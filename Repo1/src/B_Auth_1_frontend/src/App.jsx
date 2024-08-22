import React, { useState, useEffect } from 'react';
import { HttpAgent } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import logo from "../public/logo.png"

// Replace this with your canister ID
const CANISTER_ID = "bkyz2-fmaaa-aaaaa-qaaaq-cai";
const HOST = "http://localhost:8000"; // or your ICP network endpoint

const agent = new HttpAgent({ host: HOST });

const App = () => {
  const [userPrincipal, setUserPrincipal] = useState('');
  const [isProducer, setIsProducer] = useState(false);
  const [isWholesaler, setIsWholesaler] = useState(false);
  const [nftId, setNftId] = useState('');
  const [metadata, setMetadata] = useState('');
  const [batchId, setBatchId] = useState('');
  const [response, setResponse] = useState('');

  useEffect(() => {
    // Initialize the dfx agent
    agent.fetchRootKey();
  }, []);

  const addUser = async () => {
    const canister = await agent.getCanister(CANISTER_ID);
    const response = await canister.add_user(Principal.fromText(userPrincipal), isProducer, isWholesaler);
    setResponse(response);
  };

  const createBatchNFT = async () => {
    const canister = await agent.getCanister(CANISTER_ID);
    const response = await canister.create_batch_nft(metadata);
    setNftId(response);
  };

  const createProductNFT = async () => {
    const canister = await agent.getCanister(CANISTER_ID);
    const response = await canister.create_product_nft(batchId, metadata);
    setNftId(response);
  };

  const transferNFT = async () => {
    const canister = await agent.getCanister(CANISTER_ID);
    const response = await canister.transfer_nft_to_wholesaler(batchId, nftId, userPrincipal);
    setResponse(response);
  };

  return (
    <div >
      <div className='imgcontainer' id='logo'>
        <img src={logo} height="150px"alt="Bauth Logo" />
         </div>
      <h2>Production Unit Interface</h2>
      <div>
        <h2>Add User</h2>
        <input
          type="text"
          placeholder="Principal ID"
          value={userPrincipal}
          onChange={(e) => setUserPrincipal(e.target.value)}
        />
        <input
          type="checkbox"
          checked={isProducer}
          onChange={(e) => setIsProducer(e.target.checked)}
        />
        Producer
        <input
          type="checkbox"
          checked={isWholesaler}
          onChange={(e) => setIsWholesaler(e.target.checked)}
        />
        Wholesaler
        <button onClick={addUser}>Add User</button>
      </div>
      <div>
        <h2>Create Batch NFT</h2>
        <input
          type="text"
          placeholder="Metadata"
          value={metadata}
          onChange={(e) => setMetadata(e.target.value)}
        />
        <button onClick={createBatchNFT}>Create Batch NFT</button>
      </div>
      <div>
        <h2>Create Product NFT</h2>
        <input
          type="text"
          placeholder="Metadata"
          value={metadata}
          onChange={(e) => setMetadata(e.target.value)}
        />
        <input
          type="text"
          placeholder="Batch ID"
          value={batchId}
          onChange={(e) => setBatchId(e.target.value)}
        />
        <button onClick={createProductNFT}>Create Product NFT</button>
      </div>
      <div>
        <h2>Transfer NFT</h2>
        <input
          type="text"
          placeholder="Batch ID"
          value={batchId}
          onChange={(e) => setBatchId(e.target.value)}
        />
        <input
          type="text"
          placeholder="NFT ID"
          value={nftId}
          onChange={(e) => setNftId(e.target.value)}
        />
        <input
          type="text"
          placeholder="Wholesaler Principal ID"
          value={userPrincipal}
          onChange={(e) => setUserPrincipal(e.target.value)}
        />
        <button onClick={transferNFT}>Transfer NFT</button>
      </div>
      <div>
        <h2>Response</h2>
        <p>{response}</p>
      </div>
    </div>
  );
};

export default App;
