import { Group, Image, Text, rem } from "@mantine/core";
import { IconUpload, IconX } from "@tabler/icons-react";
import { Dropzone, DropzoneProps } from "@mantine/dropzone";
import prismaLogo from "../../assets/brand-prisma.svg";
import styles from "./SchemaUpload.module.css";

export function SchemaUpload(props: Partial<DropzoneProps>) {
  const onDrop: DropzoneProps["onDrop"] = async (files) => {
    let bodyContent = new FormData();
    bodyContent.append("schema", files[0]);
    let headersList = {
      Accept: "*/*",
    };
    let response = await fetch(
      "https://p8000-zc339df42-z8e3a4a05-gtw.z8daea048.qovery.fr/api/v1/visualise",
      {
        method: "POST",
        body: bodyContent,
        headers: headersList,
      }
    );
    console.log(await response.json());
  };
  return (
    <Dropzone
      onDrop={onDrop}
      onReject={(files) => console.log("rejected files", files)}
      maxSize={3 * 1024 ** 2}
      accept={["prisma"]}
      className={styles.dropZone}
      {...props}
    >
      <Group
        justify="center"
        gap="xs"
        mih={220}
        style={{ pointerEvents: "none" }}
      >
        <Dropzone.Accept>
          <IconUpload
            style={{
              width: rem(52),
              height: rem(52),
              color: "var(--mantine-color-blue-6)",
            }}
            stroke={1.5}
          />
        </Dropzone.Accept>
        <Dropzone.Reject>
          <IconX
            style={{
              width: rem(52),
              height: rem(52),
              color: "var(--mantine-color-red-6)",
            }}
            stroke={1.5}
          />
        </Dropzone.Reject>
        <Image mx={"sm"} src={prismaLogo} alt="Prisma"></Image>

        <Group gap={"xs"}>
          <Text size="xl" inline>
            Drag and drop any
          </Text>
          <Text style={{ fontWeight: 700 }} size="xl" inline>
            '.prisma'
          </Text>
          <Text size="xl" inline>
            file here
          </Text>
        </Group>
      </Group>
    </Dropzone>
  );
}
