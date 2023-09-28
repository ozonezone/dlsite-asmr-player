import { rspc } from "@/pages/_state";
import { DocumentIcon, PhotoIcon, PlayIcon } from "@heroicons/react/24/solid";
import { useSetAtom } from "jotai";
import { Link as RouterLink, useParams } from "react-router-dom";
import { playerDataAtom } from "../../_state";
import { isAudioFile, isImageFile } from "@/const";
import {
  Accordion,
  ActionIcon,
  Badge,
  Divider,
  List,
  Table,
  Tabs,
  Title,
} from "@mantine/core";
import ImageGallery from "react-image-gallery";
import "react-image-gallery/styles/css/image-gallery.css";
import { AgeBadge } from "@/components/AgeBadge";
import { Skeleton } from "@/components/Skeleton";
import { useState } from "react";
import { useStreamUrl } from "@/utils/useStreamUrl";
import { Link } from "@/components/Link";

export default function Page() {
  const { productId } = useParams();

  if (!productId) {
    return <div>Error</div>;
  }

  return <ProductInner productId={productId} />;
}

const fileSort = (a: string[], b: string[]) => {
  if (a.length == b.length) {
    for (let i = 0; i < a.length; i++) {
      if (a[i] == b[i]) {
        continue;
      }
      const aNum = a[i].match(/[0-9]+/g);
      const bNum = b[i].match(/[0-9]+/g);
      if (aNum && bNum) {
        for (let j = 0; j < Math.min(aNum.length, bNum.length); j++) {
          if (aNum[j] == bNum[j]) {
            continue;
          }
          return parseInt(aNum[j]) - parseInt(bNum[j]);
        }
      } else {
        return a[i] < b[i] ? -1 : 1;
      }
    }
  }
  return a.join("/").localeCompare(b.join("/"));
};

function ProductInner(props: { productId: string }) {
  const { data: files } = rspc.useQuery(["product.files", props.productId]);
  const { data: product } = rspc.useQuery(["product.get", props.productId]);
  const setPlayerData = useSetAtom(playerDataAtom);
  const getStreamUrl = useStreamUrl();
  const [imageIdx, setImageIdx] = useState<null | number>(null);

  const audioFiles =
    files?.filter((file) => isAudioFile(file[file.length - 1] ?? "")).sort(
      fileSort,
    ) ??
      [];
  const imageFiles =
    files?.filter((file) => isImageFile(file[file.length - 1] ?? "")).sort(
      fileSort,
    ) ??
      [];
  const otherFiles = files?.filter(
    (file) =>
      !isAudioFile(file[file.length - 1] ?? "") &&
      !isImageFile(file[file.length - 1] ?? ""),
  ).sort(fileSort) ?? [];

  return files && product
    ? (
      <div className="flex flex-col gap-3">
        <Title order={2}>{product.title}</Title>
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-2">
          <Tabs
            defaultValue={imageFiles.length == 0 ? "remote" : "local"}
            className="bgpx-3"
          >
            <Tabs.List className="pb-2">
              <Tabs.Tab value="remote">DLSite</Tabs.Tab>
              <Tabs.Tab value="local" disabled={imageFiles.length == 0}>
                Local
              </Tabs.Tab>
            </Tabs.List>

            <Tabs.Panel value="remote">
              <ImageGallery
                showPlayButton={false}
                items={product.images.map((url) => {
                  return {
                    originalHeight: 400,
                    original: url,
                    thumbnail: url,
                  };
                })}
              />
            </Tabs.Panel>
            <Tabs.Panel value="local">
              <ImageGallery
                showPlayButton={false}
                items={imageFiles.map((path) => {
                  const url = getStreamUrl(props.productId, path);
                  return {
                    originalHeight: 400,
                    original: url,
                    thumbnail: url,
                  };
                })}
              />
            </Tabs.Panel>
          </Tabs>
          <div>
            <Table>
              <Table.Tbody>
                <Table.Tr>
                  <Table.Td>DLSite</Table.Td>
                  <Table.Td>
                    <a
                      className="font-medium text-blue-600 dark:text-blue-500 hover:underline"
                      href={`https://www.dlsite.com/maniax/work/=/product_id/${product.id}.html`}
                      target="_blank"
                      referrerPolicy="no-referrer"
                    >
                      {product.id}
                    </a>
                  </Table.Td>
                </Table.Tr>
                <Table.Tr>
                  <Table.Td>対象年齢</Table.Td>
                  <Table.Td>
                    <Link to={"/?q=age:" + product.age}>
                      <AgeBadge age={product.age} />
                    </Link>
                  </Table.Td>
                </Table.Tr>
                <Table.Tr>
                  <Table.Td>販売日</Table.Td>
                  <Table.Td>{product.released_at}</Table.Td>
                </Table.Tr>
                <Table.Tr>
                  <Table.Td>サークル</Table.Td>
                  <Table.Td>
                    <Link
                      to={"/?q=circle:" +
                        product.circle.name.replaceAll(" ", "_")}
                    >
                      {product.circle.name}
                    </Link>
                  </Table.Td>
                </Table.Tr>
                <Table.Tr>
                  <Table.Td>シリーズ</Table.Td>
                  <Table.Td>{product.series ?? "-"}</Table.Td>
                </Table.Tr>
                <Table.Tr>
                  <Table.Td>声優</Table.Td>
                  <Table.Td>
                    {product.creators.filter((v) => v.role == "VoiceActor").map(
                      (v) => (
                        <Link
                          key={v.creatorName}
                          className="mr-2"
                          to={"/?q=creator:" + v.creatorName}
                        >
                          {v.creatorName}
                        </Link>
                      ),
                    )}
                  </Table.Td>
                </Table.Tr>
                <Table.Tr>
                  <Table.Td>ジャンル</Table.Td>
                  <Table.Td className="flex flex-row gap-2 flex-wrap">
                    {product.genres.map((genre) => {
                      return (
                        <RouterLink
                          key={genre.genre.name}
                          to={"/?q=genre:" + genre.genre.name}
                        >
                          <Badge variant="filled" key={genre.genreId}>
                            {genre.genre.name}
                          </Badge>
                        </RouterLink>
                      );
                    })}
                  </Table.Td>
                </Table.Tr>
              </Table.Tbody>
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
                              queue: audioFiles.map((file) => {
                                return {
                                  productId: props.productId,
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
                          <DocumentIcon className="w-4 h-4" />
                        </ActionIcon>
                      }
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
