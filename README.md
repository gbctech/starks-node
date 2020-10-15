![Web3](https://ipfs.io/ipfs/QmYxpgfvbLB5mk6fB5CtfZkex3oJfVs1b6MNJMYnbu9Mdb)

# Starks Network

## Introduction

The Starks Network aims to enrich the Polkadot/Kusama ecosystem with zero-knowledge proof technology. It can bring [zk-STARK](https://vitalik.ca/general/2017/11/09/starks_part_1.html) proof generation/verification capability to general purpose computations. The Starks Network can enable a wide range of applications from data privacy protection to novel DeFi infrastructures. 

The Starks Node is built upon the the [Substrate](https://github.com/paritytech/substrate) blockchain framework. At its core, it uses the [Distaff VM](https://github.com/GuildOfWeavers/distaff), a zk-STARK virtual machine, for STARK proof generation and verification. In the future, the Starks Network will become a parachain/parathread in Polkadot/Kusama and serve all other chains in the network via cross-chain communications. 

## Quick Demo Walkthrough

Pull the Starks Node and UI docker images.

```bash
$ docker pull starksnetwork/starks-node:0.0.3
$ docker pull starksnetwork/starks-ui:ms-1
```

Start the node container to setup a local test network. Start the ui container to interact with the node. 

```bash
$ docker run -d --name starks-ui -p 3000:80 starksnetwork/starks-ui:ms-1
$ docker run --name starks-node -p 9944:9944 starksnetwork/starks-node:0.0.3 --dev --rpc-external --ws-external
```

Open browser, enter url: `127.0.0.1:3000` to access the web app. On the top-left corner, switch to DEVELOPMENT -> Local Node (127.0.0.1:9944) in order to connect to the test network. 

Navigate in the menu: Developer -> RPC calls, call the selected endpoint `distaffvm` and use the `verify` function as shown below. 

![test-ui](https://ipfs.io/ipfs/QmSsxSvyCTmheY3TvU84YW9DuyBgBKicrwesr4ymB511yF)

There are two ways to get example proof data for verification purpose. One is to download some pre-generated [proof data](https://ipfs.io/ipfs/QmbuemBkvXpN1e1goFx5kEtA8RJS2fjyAeqCboNG8vrDfU) from IPFS, unzip and upload the files as specified in each of the four RPC fields. Then click **Submit RPC call**. You should observe "verification passed" as the output. This means the proof data, along with other data, have been sent to the Distaff VM in the substrate chain and a verification operation has been successfully performed. 

If you want to generate STARK proof data by yourself, you can clone the [Distaff VM repo](https://github.com/gbctech/distaff), compile it and run some example cases to get the proof data, like so:

```bash
$ git clone https://github.com/gbctech/distaff
$ cd distaff
$ cargo build --release
$ target/release/distaff
```

You should be able to find four text files (begin with s_) in the distaff project directory and use them as input for the RPC calls fields above. 















