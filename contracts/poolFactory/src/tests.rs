


#[cfg(test)]
mod tests {
    
    use crate::ContractError;
    use crate::contract::{instantiate, query, execute};
    use crate::msg::{InitMsg, QueryMsg,  ExecuteMsg};
    use crate::query::ConfigResponse;

    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    const  DUMMY: &str = "cosmos1y4fu4qfxxs9yg2pec9ualrgr9wxfyt77k45e55";



#[test]
fn proper_init() {
    let mut deps = mock_dependencies();
    let info = mock_info("sender", &[]);
    let msg = InitMsg {  admin: DUMMY.to_string(), pool_code_id: 0 };
    
    let res = instantiate(deps.as_mut(), mock_env(), info, msg);
        
    // println!("res: {:?}", res);
    
    assert_eq!(0, res.unwrap().messages.len());
    // it worked, let's query the state
    let cfg = query(deps.as_ref(), mock_env(), QueryMsg::Config {  }).unwrap();
    
    let value: ConfigResponse = from_binary(&cfg).unwrap();

    assert_eq!(DUMMY, value.admin);
    assert_eq!(0, value.pool_code_id);

}

fn create_a_pool(){
    let mut deps = mock_dependencies();
    let info = mock_info("sender", &[]);
    
}

fn handle_transfer_reply(){
    
}


}