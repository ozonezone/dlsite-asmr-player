import { rspc } from "@/state";
import { PhotoIcon, PlayIcon } from "@heroicons/react/24/solid";
import { useSetAtom } from "jotai";
import { useParams } from "react-router-dom";
import { playerDataAtom } from "../state";
import { isAudioFile, isImageFile } from "@/const";
import {
  Accordion,
  ActionIcon,
  Badge,
  Divider,
  List,
  Table,
  Title,
} from "@mantine/core";
import ImageGallery from "react-image-gallery";
import "react-image-gallery/styles/css/image-gallery.css";
import { AgeBadge } from "./components/AgeBadge";
import { Skeleton } from "@/components/Skeleton";
import { useEffect, useState } from "react";
import { useStreamUrl } from "../utils";

export default function Page() {
  const { productId } = useParams();

  if (!productId) {
    return <div>Error</div>;
  }

  return <ProductInner productId={productId} />;
}

function ProductInner(props: { productId: string }) {
  const { data: files } = rspc.useQuery(["product.files", props.productId]);
  const { data: product } = rspc.useQuery(["product.get", props.productId]);
  const setPlayerData = useSetAtom(playerDataAtom);
  const getStreamUrl = useStreamUrl();
  const [imageIdx, setImageIdx] = useState<null | number>(null);

  const audioFiles =
    files?.filter((file) => isAudioFile(file[file.length - 1] ?? "")).sort() ??
      [];
  const imageFiles =
    files?.filter((file) => isImageFile(file[file.length - 1] ?? "")).sort() ??
      [];
  const otherFiles = files?.filter(
    (file) =>
      !isAudioFile(file[file.length - 1] ?? "") &&
      !isImageFile(file[file.length - 1] ?? ""),
  ).sort() ?? [];

  return files && product
    ? (
      <div className="flex flex-col gap-3">
        <Title order={2}>{product.name}</Title>
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-2">
          <ImageGallery
            showPlayButton={false}
            items={product.remote_image.map((url) => {
              return {
                original: url,
                thumbnail: url,
              };
            })}
          />
          <div>
            <Table>
              <tbody>
                <tr>
                  <td>DLSite</td>
                  <td>
                    <a
                      className="font-medium text-blue-600 dark:text-blue-500 hover:underline"
                      href={`https://www.dlsite.com/maniax/work/=/product_id/${product.id}.html`}
                    >
                      {product.id}
                    </a>
                  </td>
                </tr>
                <tr>
                  <td>対象年齢</td>
                  <td>
                    <AgeBadge age={product.age} />
                  </td>
                </tr>
                <tr>
                  <td>販売日</td>
                  <td>{product.released_at}</td>
                </tr>
                <tr>
                  <td>サークル</td>
                  <td>{product.circle_name}</td>
                </tr>
                <tr>
                  <td>シリーズ</td>
                  <td>{product.series ?? "-"}</td>
                </tr>
                <tr>
                  <td>声優</td>
                  <td>{product.actor.join(", ")}</td>
                </tr>
                <tr>
                  <td>ジャンル</td>
                  <td className="flex flex-row gap-2 flex-wrap">
                    {product.genre.map((genre) => {
                      return (
                        <Badge variant="filled" key={genre.id}>
                          {genre.name}
                        </Badge>
                      );
                    })}
                  </td>
                </tr>
              </tbody>
            </Table>
          </div>
        </div>
        <Divider />
        <div className="flex flex-col gap-2">
          <Title order={3}>ファイル</Title>
          <Accordion defaultValue={["audio"]} multiple>
            <Accordion.Item value="audio">
              <Accordion.Control>音声</Accordion.Control>
              <Accordion.Panel>
                <List
                  spacing="xs"
                  size="sm"
                  center
                >
                  {audioFiles.map((file, idx) => (
                    <List.Item
                      key={idx}
                      icon={
                        <ActionIcon
                          color="teal"
                          size={24}
                          radius="xl"
                          onClick={() => {
                            setPlayerData({
                              productId: props.productId,
                              queue: files.filter((file) =>
                                isAudioFile(file[file.length - 1] ?? "")
                              )
                                .map((file) => {
                                  return {
                                    title: file[file.length - 1],
                                    path: file,
                                  };
                                }),
                              queueIdx: idx,
                              playing: true,
                            });
                          }}
                        >
                          <PlayIcon className="w-4 h-4" />
                        </ActionIcon>
                      }
                    >
                      {file.map((path) => path).join("/")}
                    </List.Item>
                  ))}
                </List>
              </Accordion.Panel>
            </Accordion.Item>

            <Accordion.Item value="image">
              <Accordion.Control>画像</Accordion.Control>
              <Accordion.Panel>
                <List
                  spacing="xs"
                  size="sm"
                  center
                >
                  {imageFiles.map((file, idx) => (
                    <List.Item
                      key={idx}
                      icon={
                        <ActionIcon
                          color="blue"
                          size={24}
                          radius="xl"
                          onClick={() => {
                            setImageIdx(idx);
                            window.open(
                              getStreamUrl(props.productId, file),
                              "_blank",
                            );
                          }}
                        >
                          <PhotoIcon className="w-4 h-4" />
                        </ActionIcon>
                      }
                    >
                      {file.map((path) => path).join("/")}
                    </List.Item>
                  ))}
                </List>
              </Accordion.Panel>
            </Accordion.Item>

            <Accordion.Item value="other">
              <Accordion.Control>その他</Accordion.Control>
              <Accordion.Panel>
                <List
                  spacing="xs"
                  size="sm"
                  center
                >
                  {otherFiles.map((file, idx) => (
                    <List.Item
                      key={idx}
                    >
                      {file.map((path) => path).join("/")}
                    </List.Item>
                  ))}
                </List>
              </Accordion.Panel>
            </Accordion.Item>
          </Accordion>
        </div>
      </div>
    )
    : <Skeleton />;
}
