# Oracle Manipulation

## Hunt Targets

- Spot price reads without averaging (direct read of `slot0` on Uniswap V3 or pool reserves on Uniswap V2).
- Missing stale-price checks on external oracle feeds (e.g. Chainlink `latestRoundData`).
- Missing L2 sequencer uptime checks where applicable.
- Euler Finance Oracle Risk Grading indicators:
  - Low liquidity pools or token market caps under target thresholds.
  - Short TWAP windows (under 30 minutes) or volatile assets.
  - Inadequate decentralization in custom node/multi-signature price updates.

## Exploit Checks

- Identify oracle source and update model.
- Prove attacker influence over observed price.
- Euler TWAP Cost of Attack Simulation:
  - Calculate the capital required to manipulate the Uniswap V3 spot price across consecutive blocks to distort the TWAP to the target.
- PoS MEV and Block Proposer Exploits:
  - Check if a validator can propose consecutive blocks to manipulate the price feed without arbitrageurs reversing the trade (Multi-block MEV).
- Prove downstream economic consequence (vault drainage, liquidation threshold distortion).

## Reject Conditions

- Robust oracle guards active (TWAP window > 30-60 mins, Chainlink deviation thresholds, multi-source fallback).
- Attacker cannot meaningfully influence source.
- No practical impact path (liquidation yields no extractable profit).

## Evidence Required

- Vulnerable read site and lack of price smoothing/averaging.
- Manipulated value effect on protocol logic.
- Postmortem references:
  - **BONQDAO ($120M)**: Tellor oracle price manipulation where minimal TLOS was used to inflate WALBT collateral.
  - **Harvest Finance ($34M)**: Curve Y-pool manipulation via flash loan.
  - **Value DeFi ($6M)**: AMM spot read manipulation.


