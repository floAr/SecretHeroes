interface NetworkConfig {
  chainId: string;
  chainName: string;
  rpc: string;
  rest: string;
  bip44: Bip44;
  coinType: number;
  stakeCurrency: StakeCurrency;
  bech32Config: Bech32Config;
  currencies: StakeCurrency[];
  feeCurrencies: StakeCurrency[];
  gasPriceStep: GasPriceStep;
  features: string[];
}

interface GasPriceStep {
  low: number;
  average: number;
  high: number;
}

interface Bech32Config {
  bech32PrefixAccAddr: string;
  bech32PrefixAccPub: string;
  bech32PrefixValAddr: string;
  bech32PrefixValPub: string;
  bech32PrefixConsAddr: string;
  bech32PrefixConsPub: string;
}

interface StakeCurrency {
  coinDenom: string;
  coinMinimalDenom: string;
  coinDecimals: number;
}

interface Bip44 {
  coinType: number;
}
