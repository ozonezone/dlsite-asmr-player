import AudioPlayer from "react-h5-audio-player";
import "react-h5-audio-player/lib/styles.css";
import { PlayerData } from "../state";
import { useAtomValue } from "jotai";
import { authAtom } from "@/state";
import { SERVER_HOST } from "@/const";

export function Player(props: { playerData: NonNullable<PlayerData> }) {
  const token = useAtomValue(authAtom)!;
  const currentFilePath =
    props.playerData.queue[props.playerData.queueIdx].path;
  return (
    <AudioPlayer
      autoPlay
      src={`http://${SERVER_HOST}/stream/${props.playerData.productId}/${
        currentFilePath.join("/")
      }?token=${token}`}
    />
  );
}
