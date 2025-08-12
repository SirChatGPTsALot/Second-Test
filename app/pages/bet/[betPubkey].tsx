import { useRouter } from "next/router";

export default function BetDetail() {
  const router = useRouter();
  const { betPubkey } = router.query;

  return <div>Bet detail for {betPubkey?.toString()}</div>;
}
