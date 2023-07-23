import AudioPlayer from "react-h5-audio-player";
import "react-h5-audio-player/lib/styles.css";
import { useSetAtom } from "jotai";
import { ActionIcon } from "@mantine/core";
import { XMarkIcon } from "@heroicons/react/24/solid";

import { PlayerData, playerDataAtom } from "../state";
import { useStreamUrl } from "../utils";
import { useEffect, useRef } from "react";
import H5AudioPlayer from "react-h5-audio-player";
import { rspc } from "@/state";

export function Player(props: { playerData: NonNullable<PlayerData> }) {
  const setPlayerData = useSetAtom(playerDataAtom);
  const getStreamUrl = useStreamUrl();
  const currentFile = props.playerData.queue[props.playerData.queueIdx];
  const currentFileName = currentFile.path[currentFile.path.length - 1];
  const currentFileParents = currentFile.path.slice(0, -1);
  const player = useRef<H5AudioPlayer>(null);
  const { data: product } = rspc.useQuery([
    "product.get",
    currentFile.productId,
  ]);

  const goNext = () => {
    if (props.playerData.queueIdx + 1 < props.playerData.queue.length) {
      setPlayerData({
        ...props.playerData,
        queueIdx: props.playerData.queueIdx + 1,
      });
    }
  };

  const goPrevious = () => {
    const audio = player?.current?.audio.current;
    if (audio?.currentTime && audio.currentTime < 2) {
      if (props.playerData.queueIdx - 1 >= 0) {
        setPlayerData({
          ...props.playerData,
          queueIdx: props.playerData.queueIdx - 1,
        });
      }
    } else {
      if (audio?.currentTime) {
        audio.currentTime = 0;
      }
    }
  };

  useEffect(() => {
    navigator.mediaSession.setActionHandler("previoustrack", () => {
      goPrevious();
    });
    navigator.mediaSession.setActionHandler("nexttrack", () => {
      goNext();
    });

    return () => {
      navigator.mediaSession.setActionHandler("nexttrack", null);
    };
  });

  return (
    product
      ? (
        <AudioPlayer
          autoPlay
          ref={player}
          showSkipControls={true}
          header={
            <div className="flex flex-row justify-between">
              <div>
                <span className="text-gray-400">
                  {currentFile.productId}/
                </span>
                {currentFileParents.map((path, idx) => (
                  <span className="text-gray-400" key={idx}>
                    {path}/
                  </span>
                ))}
                <span>{currentFileName}</span>
              </div>
              <ActionIcon onClick={() => setPlayerData(null)}>
                <XMarkIcon />
              </ActionIcon>
            </div>
          }
          src={getStreamUrl(currentFile.productId, currentFile.path)}
          onClickNext={() => {
            goNext();
          }}
          onEnded={() => {
            goNext();
          }}
          onPlay={() => {
            navigator.mediaSession.metadata = new MediaMetadata({
              title: currentFileName,
              album: product.product.name,
              artist: product.circle_name,
            });
          }}
          onListen={() => {
            const audio = player?.current?.audio.current;
            if (audio) {
              navigator.mediaSession.setPositionState({
                duration: audio.duration,
                position: audio.currentTime,
              });
            }
          }}
          onClickPrevious={() => {
            goPrevious();
          }}
        />
      )
      : <></>
  );
}
