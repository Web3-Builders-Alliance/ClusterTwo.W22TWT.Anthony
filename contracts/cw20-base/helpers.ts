/*
 * This is a set of helpers meant for use with @cosmjs/cli
 * With these you can easily use the cw20 contract without worrying about forming messages and parsing queries.
 * 
 * Usage: npx @cosmjs/cli --init https://github.com/CosmWasm/cosmwasm-plus/blob/master/contracts/cw20-base/helpers.ts
 * 
 * Create a client:
 *   const client = await useOptions(coralnetOptions).setup(password);
 *   await client.getAccount()
 * 
 * Get the mnemonic:
 *   await useOptions(coralnetOptions).recoverMnemonic(password)
 * 
 * If you want to use this code inside an app, you will need several imports from https://github.com/CosmWasm/cosmjs
 */

const path = require("path");

interface Options {
  readonly httpUrl: string
  readonly networkId: string
  readonly feeToken: string
  readonly gasPrice: number
  readonly bech32prefix: string
  readonly hdPath: readonly Slip10RawIndex[]
  readonly faucetToken: string
  readonly faucetUrl?: string
  readonly defaultKeyFile: string
}
  
const coralnetOptions: Options = {
  httpUrl: 'https://lcd.coralnet.cosmwasm.com',
  networkId: 'cosmwasm-coral',
  feeToken: 'ushell',
  gasPrice: 0.025,
  bech32prefix: 'coral',
  faucetToken: 'SHELL',
  faucetUrl: 'https://faucet.coralnet.cosmwasm.com/credit',
  hdPath: makeCosmoshubPath(0),
  defaultKeyFile: path.join(process.env.HOME, ".coral.key"),
}

interface Network {
  setup: (password: string, filename?: string) => Promise<SigningCosmWasmClient>
  recoverMnemonic: (password: string, filename?: string) => Promise<string>
}

const useOptions = (options: Options): Network => {

  const loadOrCreateWallet = async (options: Options, filename: string, password: string): Promise<Secp256k1Wallet> => {
    try {
      const encrypted = fs.readFileSync(filename, 'utf8');
      const wallet = await Secp256k1Wallet.deserialize(encrypted, password);
      return wallet;
    } catch (err) {
      const wallet = await Secp256k1Wallet.generate(12, options.hdPath, options.bech32prefix);
      const encrypted = await wallet.serialize(password);
      fs.writeFileSync(filename, encrypted, 'utf8');
      return wallet;
    }
  };
  
  const buildFeeTable = (options: Options): FeeTable => {
    const { feeToken, gasPrice } = options;
    const stdFee = (gas: number, denom: string, price: number) => {
      const amount = Math.floor(gas * price)
      return {
        amount: [{ amount: amount.toString(), denom: denom }],
        gas: gas.toString(),
      }
    }
  
    return {
      upload: stdFee(1500000, feeToken, gasPrice),
      init: stdFee(600000, feeToken, gasPrice),
      migrate: stdFee(600000, feeToken, gasPrice),
      exec: stdFee(200000, feeToken, gasPrice),
      send: stdFee(80000, feeToken, gasPrice),
      changeAdmin: stdFee(80000, feeToken, gasPrice),
    }
  };

  const connect = async (
    wallet: Secp256k1Wallet,
    options: Options
  ): Promise<SigningCosmWasmClient> => {
    const feeTable = buildFeeTable(options);
    const [{ address }] = await wallet.getAccounts();
  
    const client = new SigningCosmWasmClient(
      options.httpUrl,
      address,
      wallet,
      feeTable
    );
    return client;
  };
  
  const hitFaucet = async (
    faucetUrl: string,
    address: string,
    ticker: string
  ): Promise<void> => {
    await axios.post(faucetUrl, { ticker, address });
  }
  
  const setup = async (password: string, filename?: string): Promise<SigningCosmWasmClient> => {
    const keyfile = filename || options.defaultKeyFile;
    const wallet = await loadOrCreateWallet(coralnetOptions, keyfile, password);
    const client = await connect(wallet, coralnetOptions);

    // ensure we have some tokens
    if (options.faucetUrl) {
      const account = await client.getAccount();
      if (!account) {
        console.log(`Getting ${options.feeToken} from faucet`);
        await hitFaucet(options.faucetUrl, client.senderAddress, options.faucetToken);
      }  
    }

    return client;
  }

  const recoverMnemonic = async (password: string, filename?: string): Promise<string> => {
    const keyfile = filename || options.defaultKeyFile;
    const wallet = await loadOrCreateWallet(coralnetOptions, keyfile, password);
    return wallet.mnemonic;
  }

  return {setup, recoverMnemonic};
}

interface Balances {
  readonly address: string
  readonly amount: string  // decimal as string
}

interface MintInfo {
  readonly minter: string
  readonly cap?: string // decimal as string
}

interface InitMsg {
  readonly name: string
  readonly symbol: string
  readonly decimals: number
  readonly initial_balances: readonly Balances[]
  readonly mint?: MintInfo
}

interface CW20Instance {
  readonly contractAddress: string

  // queries
  balance: (address?: string) => Promise<string>
  allowance: (owner: string, spender: string) => Promise<string>
  token_info: () => Promise<any>
  minter: () => Promise<any>

  // actions
  mint: (recipient: string, amount: string) => Promise<string>
  transfer: (recipient: string, amount: string) => Promise<string>
  burn: (amount: string) => Promise<string>
  increaseAllowance: (recipient: string, amount: string) => Promise<string>
  decreaseAllowance: (recipient: string, amount: string) => Promise<string>
  transferFrom: (owner: string, recipient: string, amount: string) => Promise<string>
}

interface CW20Contract {
  // upload a code blob and returns a codeId
  upload: () => Promise<number>

  // instantiates a cw20 contract
  // codeId must come from a previous deploy
  // label is the public name of the contract in listing
  // if you set admin, you can run migrations on this contract (likely client.senderAddress)
  instantiate: (codeId: number, initMsg: InitMsg, label: string, admin?: string) => Promise<CW20Instance>

  use: (contractAddress: string) => CW20Instance
}


const CW20 = (client: SigningCosmWasmClient): CW20Contract => {
  const use = (contractAddress: string): CW20Instance => {
    const balance = async (account?: string): Promise<string> => {
      const address = account || client.senderAddress;  
      const result = await client.queryContractSmart(contractAddress, {balance: { address }});
      return result.balance;
    };

    const allowance = async (owner: string, spender: string): Promise<string> => {
      const result = await client.queryContractSmart(contractAddress, {allowance: { owner, spender }});
      return result.allowance;
    };

    const token_info = async (): Promise<any> => {
      return client.queryContractSmart(contractAddress, {token_info: { }});
    };

    const minter = async (): Promise<any> => {
      return client.queryContractSmart(contractAddress, {minter: { }});
    };

    // mints tokens, returns transactionHash
    const mint = async (recipient: string, amount: string): Promise<string> => {
      const result = await client.execute(contractAddress, {mint: {recipient, amount}});
      return result.transactionHash;
    }

    // transfers tokens, returns transactionHash
    const transfer = async (recipient: string, amount: string): Promise<string> => {
      const result = await client.execute(contractAddress, {transfer: {recipient, amount}});
      return result.transactionHash;
    }

    // burns tokens, returns transactionHash
    const burn = async (amount: string): Promise<string> => {
      const result = await client.execute(contractAddress, {burn: {amount}});
      return result.transactionHash;
    }

    const increaseAllowance = async (spender: string, amount: string): Promise<string> => {
      const result = await client.execute(contractAddress, {increase_allowance: {spender, amount}});
      return result.transactionHash;
    }

    const decreaseAllowance = async (spender: string, amount: string): Promise<string> => {
      const result = await client.execute(contractAddress, {decrease_allowance: {spender, amount}});
      return result.transactionHash;
    }

    const transferFrom = async (owner: string, recipient: string, amount: string): Promise<string> => {
      const result = await client.execute(contractAddress, {transfer_from: {owner, recipient, amount}});
      return result.transactionHash;
    }
    
    return {
      contractAddress,
      balance,
      allowance,
      token_info,
      minter,
      mint,
      transfer,
      burn,
      increaseAllowance,
      decreaseAllowance,
      transferFrom,
    };
  }

  const downloadWasm = async (url: string): Promise<Uint8Array> => {
    const r = await axios.get(url, { responseType: 'arraybuffer' })
    if (r.status !== 200) {
      throw new Error(`Download error: ${r.status}`)
    }
    return r.data
  }
  
  const upload = async (): Promise<number> => {
    const meta = {
      source: "https://github.com/CosmWasm/cosmwasm-plus",
      builder: "cosmwasm/rust-optimizer:0.10.1"
    };
    const sourceUrl = "https://github.com/CosmWasm/cosmwasm-plus/releases/download/v0.1.1/cw20_base.wasm";
    const wasm = await downloadWasm(sourceUrl);
    const result = await client.upload(wasm, meta);
    return result.codeId;
  }

  const instantiate = async (codeId: number, initMsg: InitMsg, label: string, admin?: string): Promise<CW20Instance> => {
    const result = await client.instantiate(codeId, initMsg, label, { memo: `Init ${label}`, admin});
    return use(result.contractAddress);
  }

  return { upload, instantiate, use };
}


/*** this is demo code  ***/
const demo = async () => {
  console.log("Running demo....");
  const client = await useOptions(coralnetOptions).setup("12345678");
  console.log(client.senderAddress);
  const account = await client.getAccount();
  console.log(account);
}

const exampleStars = async () => {
  console.log("Setup....");
  const client = await useOptions(coralnetOptions).setup("12345678");

  const cw20 = CW20(client);
  const codeId = await cw20.upload();
  console.log(`CodeId: ${codeId}`);

  const initMsg: InitMsg = {
    name: "Golden Stars",
    symbol: "STAR",
    decimals: 2,
    // list of all validator self-delegate addresses - 100 STARs each!
    initial_balances: [
      { address: "coral1exta8hzrghyt5umd4jh55kfkmp0tv3hyg8krc5", amount: "10000"},
      { address: "coral13mcejut8e5tncs59zcs4yn4envcd98vx682frk", amount: "10000"},
      { address: "coral10zn0d2eeust0495crtr3zqz7t688hg0s53afrh", amount: "10000"},
      { address: "coral1qvrcashqpemlkhrqphzv9n5nutdxpafmdefgcl", amount: "10000"},
      { address: "coral14f8nvyy4c9pyn78dgv0k6syek3jjjrkyz747kj", amount: "10000"},
      { address: "coral1e86v774dch5uwkks0cepw8mdz8a9flhhapvf6w", amount: "10000"},
      { address: "coral1hf50trj7plz2sd8cmcvn7c8ruh3tjhc2nhyl7l", amount: "10000"},
    ],
    mint: {
      minter: client.senderAddress,
    },
  };
  const contract = await cw20.instantiate(codeId, initMsg, "STAR");
  console.log(`Contract: ${contract.contractAddress}`);

  console.log(await contract.balance("coral13mcejut8e5tncs59zcs4yn4envcd98vx682frk"));
  console.log(await contract.balance());

  // Setup....
  // CodeId: 4
  // Contract: coral16t7y0vrtpqjw2d7jvc2209yan9002339mg4mrv
  // 10000
  // 0
}

const usage = async () => {
  const addr = "coral16t7y0vrtpqjw2d7jvc2209yan9002339mg4mrv";
  const client = await useOptions(coralnetOptions).setup("12345678");
  const stars = CW20(client).use(addr);
  
  console.log(`info: ${JSON.stringify(await stars.token_info())}`);
  console.log(`minter: ${JSON.stringify(await stars.minter())}`);
  const acct = "coral14f8nvyy4c9pyn78dgv0k6syek3jjjrkyz747kj";
  console.log(`balance of ${acct}: ${await stars.balance(acct)}`);

  console.log(`my balance: ${await stars.balance()}`);
  console.log("minting myself 100 STAR");
  const mintTx = await stars.mint(client.senderAddress, "10000");
  console.log(`Tx: ${mintTx}`);
  console.log(`my balance: ${await stars.balance()}`);

  const lucky = "coral1hf50trj7plz2sd8cmcvn7c8ruh3tjhc2nhyl7l";
  console.log(`balance of ${lucky}: ${await stars.balance(lucky)}`);
  console.log("send 5 STAR to ${lucky}");
  const transferTx = await stars.transfer(lucky, "500");
  console.log(`Tx: ${transferTx}`);
  console.log(`balance of ${lucky}: ${await stars.balance(lucky)}`);
  console.log(`my balance: ${await stars.balance()}`);

  // info: {"name":"Golden Stars","symbol":"STAR","decimals":2,"total_supply":"70000"}
  // minter: {"minter":"coral15m4z2650nkcr7r6g5dyzf4qwcrcmrrjh6t7x0f","cap":null}
  // balance of coral14f8nvyy4c9pyn78dgv0k6syek3jjjrkyz747kj: 10000
  // my balance: 0
  // minting myself 100 STAR
  // Tx: 257283B98DB5D10412839ACC9667E0E5FFF001CC1BE4AFA4527157082C15F2FA
  // my balance: 10000
  // balance of coral1hf50trj7plz2sd8cmcvn7c8ruh3tjhc2nhyl7l: 10000
  // send 5 STAR to ${lucky}
  // Tx: 83FBD409BFEBF62AB6926C592788EF7DC378CBBFA1337A33931F45F84D79B17B
  // balance of coral1hf50trj7plz2sd8cmcvn7c8ruh3tjhc2nhyl7l: 10500
  // my balance: 9500
}