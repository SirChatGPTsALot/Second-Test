import { useAnchorWallet } from "@solana/wallet-adapter-react";
import { Program } from "@project-serum/anchor";
import { useState } from "react";
// Placeholder IDL import
import idl from "../idl/bet_jury.json";

const PROGRAM_ID = "BetJury11111111111111111111111111111111111";

export default function CreateBetPage() {
  const wallet = useAnchorWallet();
  const [terms, setTerms] = useState("");
  const [stake, setStake] = useState("");

  const createBet = async () => {
    if (!wallet) return;
    const provider = wallet.provider;
    const program = new Program(idl as any, PROGRAM_ID, provider);
    // Placeholder instruction call
    await program.methods
      .createBet(terms, parseInt(stake))
      .accounts({ bet: undefined, creator: provider.wallet.publicKey })
      .rpc();
  };

  return (
    <div>
      <input
        value={terms}
        onChange={(e) => setTerms(e.target.value)}
        placeholder="Terms"
      />
      <input
        value={stake}
        onChange={(e) => setStake(e.target.value)}
        placeholder="Stake"
      />
      <button onClick={createBet}>Create Bet</button>
    </div>
  );
}
