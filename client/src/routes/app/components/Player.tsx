import AudioPlayer from "react-h5-audio-player";
import "react-h5-audio-player/lib/styles.css";
import { useAtomValue, useSetAtom } from "jotai";
import { authAtom } from "@/state";
import { SERVER_HOST } from "@/const";
import { ActionIcon } from "@mantine/core";
import { XMarkIcon } from "@heroicons/react/24/solid";

import { PlayerData, playerDataAtom } from "../state";

export function Player(props: { playerData: NonNullable<PlayerData> }) {
  const token = useAtomValue(authAtom)!;
  const setPlayerData = useSetAtom(playerDataAtom);
  const currentFile = props.playerData.queue[props.playerData.queueIdx];
  return (
    <AudioPlayer
      autoPlay
      showSkipControls={true}
      header={
        <div className="flex flex-row justify-between">
          <div>{currentFile.title}</div>
          <ActionIcon onClick={() => setPlayerData(null)}>
            <XMarkIcon />
          </ActionIcon>
        </div>
      }
      src={`http://${SERVER_HOST}/stream/${props.playerData.productId}/${
        currentFile.path.join("/")
      }?token=${token}`}
      onClickNext={() => {
        if (props.playerData.queueIdx + 1 < props.playerData.queue.length) {
          setPlayerData({
            ...props.playerData,
            queueIdx: props.playerData.queueIdx + 1,
          });
        }
      }}
      onClickPrevious={() => {
        if (props.playerData.queueIdx - 1 >= 0) {
          setPlayerData({
            ...props.playerData,
            queueIdx: props.playerData.queueIdx - 1,
          });
        }
      }}
    />
  );
}
