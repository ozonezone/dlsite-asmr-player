import { rspc } from "@/state";
import { PlayIcon } from "@heroicons/react/24/solid";
import { Button } from "flowbite-react";
import { useSetAtom } from "jotai";
import { useParams } from "react-router-dom";
import { playerDataAtom } from "../state";
import { isAudioFile } from "@/const";

export default function Page() {
  const { productId } = useParams();

  if (!productId) {
    return <div>Error</div>;
  }

  return <ProductInner productId={productId} />;
}

function ProductInner(props: { productId: string }) {
  const { data: files } = rspc.useQuery(["product.files", props.productId]);
  const setPlayerData = useSetAtom(playerDataAtom);

  return files
    ? (
      <div className="flex flex-col">
        <button
          type="button"
          onClick={() => {
            setPlayerData({
              productId: props.productId,
              queue: files.filter((file) =>
                isAudioFile(file[file.length - 1] ?? "")
              )
                .map((file) => {
                  return { title: file[file.length - 1], path: file };
                }),
              queueIdx: 0,
              playing: true,
            });
          }}
          className="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-full text-sm p-2.5 text-center inline-flex items-center mr-2 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
        >
          <PlayIcon className="h-6 w-6" />
          <span className="sr-only">Play</span>
        </button>
        <div className="flex flex-col">
          {files.map((file, idx) => <div key={idx}>{file.join("/")}</div>)}
        </div>
      </div>
    )
    : <div>Loading</div>;
}
