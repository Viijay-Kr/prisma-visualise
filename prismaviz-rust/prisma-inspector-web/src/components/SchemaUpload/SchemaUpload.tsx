import { Button, FileButton, Group, Image, Text, rem } from "@mantine/core";
import { IconUpload, IconX } from "@tabler/icons-react";
import { Dropzone, DropzoneProps, FileWithPath } from "@mantine/dropzone";
import prismaLogo from "../../assets/brand-prisma.svg";
import styles from "./SchemaUpload.module.css";
import { MutationFunction, useMutation } from "@tanstack/react-query";

const parseSchemaFile: MutationFunction<unknown, FileWithPath[]> = async (
  files
) => {
  let bodyContent = new FormData();
  bodyContent.append("schema", files[0]);
  let headersList = {
    Accept: "*/*",
  };
  const response = await fetch(
    `${import.meta.env.VITE_PRISMA_API_URL}/api/v1/visualise`,
    {
      method: "POST",
      body: bodyContent,
      headers: headersList,
    }
  );
  return await response.json();
};
export function SchemaUpload(props: Partial<DropzoneProps>) {
  const { mutateAsync, data } = useMutation<unknown, unknown, FileWithPath[]>(
    ["schema_cache"],
    parseSchemaFile
  );
  if (data) {
    return (
      <FileButton
        accept={"prisma"}
        onChange={(file) => file && mutateAsync([file])}
      >
        {(props) => (
          <Button style={{ marginLeft: "auto" }} {...props}>
            Upload another
          </Button>
        )}
      </FileButton>
    );
  }
  return (
    <Dropzone
      onDrop={mutateAsync}
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
