#[cfg(test)]
use super::*;
use crate::test_helpers::*;

use cosmwasm_std::testing::{mock_env, mock_info, MOCK_CONTRACT_ADDR};
use cosmwasm_std::{coin, coins, CosmosMsg, IbcMsg, StdError, Uint128};

use crate::state::ChannelState;
use cw_utils::PaymentError;

#[test]
fn setup_and_query() {   
    assert_eq!(err, StdError::not_found("cw20_ics20::state::ChannelInfo"));
}

#[test]
fn proper_checks_on_execute_native() {
    let send_channel = "channel-5";
    let mut deps = setup(&[send_channel, "channel-10"], &[]);

    let mut transfer = TransferMsg {
        channel: send_channel.to_string(),
        remote_address: "foreign-address".to_string(),
        timeout: None,
    };

    // works with proper funds
    let msg = ExecuteMsg::Transfer(transfer.clone());
    let info = mock_info("foobar", &coins(1234567, "ucosm"));
    let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    assert_eq!(res.messages[0].gas_limit, None);
    assert_eq!(1, res.messages.len());
    if let CosmosMsg::Ibc(IbcMsg::SendPacket {
        channel_id,
        data,
        timeout,
    }) = &res.messages[0].msg
    {
        let expected_timeout = mock_env().block.time.plus_seconds(DEFAULT_TIMEOUT);
        assert_eq!(timeout, &expected_timeout.into());
        assert_eq!(channel_id.as_str(), send_channel);
        let msg: Ics20Packet = from_binary(data).unwrap();
        assert_eq!(msg.amount, Uint128::new(1234567));
        assert_eq!(msg.denom.as_str(), "ucosm");
        assert_eq!(msg.sender.as_str(), "foobar");
        assert_eq!(msg.receiver.as_str(), "foreign-address");
    } else {
        panic!("Unexpected return message: {:?}", res.messages[0]);
    }

    // reject with no funds
    let msg = ExecuteMsg::Transfer(transfer.clone());
    let info = mock_info("foobar", &[]);
    let err = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
    assert_eq!(err, ContractError::Payment(PaymentError::NoFunds {}));

    // reject with multiple tokens funds
    let msg = ExecuteMsg::Transfer(transfer.clone());
    let info = mock_info("foobar", &[coin(1234567, "ucosm"), coin(54321, "uatom")]);
    let err = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
    assert_eq!(err, ContractError::Payment(PaymentError::MultipleDenoms {}));

    // reject with bad channel id
    transfer.channel = "channel-45".to_string();
    let msg = ExecuteMsg::Transfer(transfer);
    let info = mock_info("foobar", &coins(1234567, "ucosm"));
    let err = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
    assert_eq!(
        err,
        ContractError::NoSuchChannel {
            id: "channel-45".to_string()
        }
    );
}

#[test]
fn proper_checks_on_execute_cw20() {
    let send_channel = "channel-15";
    let cw20_addr = "my-token";
    let mut deps = setup(&["channel-3", send_channel], &[(cw20_addr, 123456)]);

    let transfer = TransferMsg {
        channel: send_channel.to_string(),
        remote_address: "foreign-address".to_string(),
        timeout: Some(7777),
    };
    let msg = ExecuteMsg::Receive(Cw20ReceiveMsg {
        sender: "my-account".into(),
        amount: Uint128::new(888777666),
        msg: to_binary(&transfer).unwrap(),
    });

    // works with proper funds
    let info = mock_info(cw20_addr, &[]);
    let res = execute(deps.as_mut(), mock_env(), info, msg.clone()).unwrap();
    assert_eq!(1, res.messages.len());
    assert_eq!(res.messages[0].gas_limit, None);
    if let CosmosMsg::Ibc(IbcMsg::SendPacket {
        channel_id,
        data,
        timeout,
    }) = &res.messages[0].msg
    {
        let expected_timeout = mock_env().block.time.plus_seconds(7777);
        assert_eq!(timeout, &expected_timeout.into());
        assert_eq!(channel_id.as_str(), send_channel);
        let msg: Ics20Packet = from_binary(data).unwrap();
        assert_eq!(msg.amount, Uint128::new(888777666));
        assert_eq!(msg.denom, format!("cw20:{}", cw20_addr));
        assert_eq!(msg.sender.as_str(), "my-account");
        assert_eq!(msg.receiver.as_str(), "foreign-address");
    } else {
        panic!("Unexpected return message: {:?}", res.messages[0]);
    }

    // reject with tokens funds
    let info = mock_info("foobar", &coins(1234567, "ucosm"));
    let err = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
    assert_eq!(err, ContractError::Payment(PaymentError::NonPayable {}));
}

#[test]
fn execute_cw20_fails_if_not_whitelisted_unless_default_gas_limit() {
    let send_channel = "channel-15";
    let mut deps = setup(&[send_channel], &[]);

    let cw20_addr = "my-token";
    let transfer = TransferMsg {
        channel: send_channel.to_string(),
        remote_address: "foreign-address".to_string(),
        timeout: Some(7777),
    };
    let msg = ExecuteMsg::Receive(Cw20ReceiveMsg {
        sender: "my-account".into(),
        amount: Uint128::new(888777666),
        msg: to_binary(&transfer).unwrap(),
    });

    // rejected as not on allow list
    let info = mock_info(cw20_addr, &[]);
    let err = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap_err();
    assert_eq!(err, ContractError::NotOnAllowList);

    // add a default gas limit
    migrate(
        deps.as_mut(),
        mock_env(),
        MigrateMsg {
            default_gas_limit: Some(123456),
        },
    )
    .unwrap();

    // try again
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();
}

#[test]
fn v3_migration_works() {
    // basic state with one channel
    let send_channel = "channel-15";
    let cw20_addr = "my-token";
    let native = "ucosm";
    let mut deps = setup(&[send_channel], &[(cw20_addr, 123456)]);

    // mock that we sent some tokens in both native and cw20 (TODO: cw20)
    // balances set high
    deps.querier
        .update_balance(MOCK_CONTRACT_ADDR, coins(50000, native));
    // pretend this is an old contract - set version explicitly
    set_contract_version(deps.as_mut().storage, CONTRACT_NAME, MIGRATE_VERSION_3).unwrap();

    // channel state a bit lower (some in-flight acks)
    let state = ChannelState {
        // 14000 not accounted for (in-flight)
        outstanding: Uint128::new(36000),
        total_sent: Uint128::new(100000),
    };
    CHANNEL_STATE
        .save(deps.as_mut().storage, (send_channel, native), &state)
        .unwrap();

    // run migration
    migrate(
        deps.as_mut(),
        mock_env(),
        MigrateMsg {
            default_gas_limit: Some(123456),
        },
    )
    .unwrap();

    // check new channel state
    let chan = query_channel(deps.as_ref(), send_channel.into()).unwrap();
    assert_eq!(chan.balances, vec![Amount::native(50000, native)]);
    assert_eq!(chan.total_sent, vec![Amount::native(114000, native)]);

    // check config updates
    let config = query_config(deps.as_ref()).unwrap();
    assert_eq!(config.default_gas_limit, Some(123456));
}