import { Button, Flex, Grid, Table, Text } from "@mantine/core";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useEffect, useState } from "react";
import { SchemaResult } from "../../types";
import {
  IconCode,
  IconMaximizeOff,
  IconRelationManyToMany,
  IconTable,
} from "@tabler/icons-react";
import { Tooltip } from "../Tooltip/Tooltip";
import "./DisplaySchema.css";
import DomPurify from "dompurify";

export const DisplaySchema = () => {
  const queryClient = useQueryClient();
  const mutationCache = queryClient.getMutationCache();
  const [activeSchema, setActiveSchema] = useState<SchemaResult>();
  const [activeModel, setActiveModel] = useState<string>("");
  useEffect(() => {
    const unsubscribe = mutationCache.subscribe((event) => {
      if (event.mutation?.options.mutationKey?.includes("schema_cache")) {
        setActiveSchema(event.mutation.state.data);
      }
    });
    return () => {
      unsubscribe();
    };
  }, [mutationCache]);

  return activeSchema ? (
    <Grid mt={"md"} gutter={"xs"}>
      {activeSchema.result.map((model) => (
        <Model
          activeModel={activeModel}
          setActiveModel={setActiveModel}
          model={model}
          schema={activeSchema.schema}
        />
      ))}
    </Grid>
  ) : null;
};

const Model = ({
  model,
  setActiveModel,
  activeModel,
  schema,
}: {
  model: SchemaResult["result"][number];
  setActiveModel: (id: string) => void;
  activeModel: string;
  schema: SchemaResult["schema"];
}) => {
  const codeHighlight = useMutation<
    { code: { html: string } },
    unknown,
    { code: string }
  >(
    [`code_highlight_${model.id}`],
    async () => {
      const response = await fetch(
        `${import.meta.env.VITE_PRISMA_API_URL}/api/v1/code_highlight`,
        {
          method: "POST",
          body: JSON.stringify({ span: model.span, schema }),
          headers: {
            "Content-Type": "application/json",
          },
        }
      );
      return await response.json();
    },
    {
      onSuccess() {
        setDisplayAs("code");
      },
    }
  );
  const CodeIcon = () => (
    <Tooltip label={"Code"}>
      <Button
        onClick={() => {
          codeHighlight.mutate({
            code: model.code,
          });
        }}
        size="xs"
        variant="transparent"
        p={"0"}
      >
        <IconCode color={displayAs === "code" ? "white" : "#CC5DE8"}></IconCode>
      </Button>
    </Tooltip>
  );

  const TableIcon = () => (
    <Tooltip label={"Table"}>
      <Button
        size="xs"
        variant="transparent"
        p={"0"}
        onClick={() => {
          setDisplayAs("table");
        }}
      >
        <IconTable
          color={displayAs === "table" ? "white" : "#CC5DE8"}
        ></IconTable>
      </Button>
    </Tooltip>
  );

  const CollapsedIcon = () => (
    <Tooltip label="Collapse">
      <Button
        size="xs"
        variant="transparent"
        p={"0"}
        onClick={() => {
          setDisplayAs("collapsed");
          if (activeModel === model.id) {
            setActiveModel("");
          }
        }}
      >
        <IconMaximizeOff
          color={displayAs === "collapsed" ? "white" : "#CC5DE8"}
        ></IconMaximizeOff>
      </Button>
    </Tooltip>
  );

  const RelationShipIcon = () => {
    return (
      <Tooltip label="Describe Relationships">
        <Button size="xs" variant="transparent" p={"0"}>
          <IconRelationManyToMany color="#CC5DE8"></IconRelationManyToMany>
        </Button>
      </Tooltip>
    );
  };

  const ModelCaption = () => (
    <Flex
      style={{
        textAlign: "left",
        marginBottom: "0px",
        padding: "8px",
        fontSize: "15px",
        fontWeight: 700,
        color: "white",
        borderTopLeftRadius: "8px",
        borderTopRightRadius: "8px",
      }}
      bg={"dark.5"}
    >
      {model.name}
    </Flex>
  );

  const DisplayAsCode = () => (
    <Flex
      style={{
        fontSize: "16px",
        fontFamily: "Space mono, monospace",
        color: "#845EF7",
        borderRadius: 0,
      }}
      bg="dark.9"
      p={"md"}
      dangerouslySetInnerHTML={{
        __html: DomPurify.sanitize(codeHighlight.data?.code.html ?? ""),
      }}
    />
  );

  const DisplayAsTable = () => (
    <Table
      captionSide="top"
      striped
      highlightOnHover
      withTableBorder
      withColumnBorders
    >
      <Table.Thead>
        <Table.Tr>
          <Table.Th>Field name</Table.Th>
          <Table.Th>Type</Table.Th>
          <Table.Th>Constraints</Table.Th>
          <Table.Th>Relationship Field</Table.Th>
          <Table.Th>Relationship Reference</Table.Th>
          <Table.Th>Is_Index</Table.Th>
        </Table.Tr>
      </Table.Thead>
      <Table.Tbody>
        {model.fields.map((field) => (
          <Table.Tr key={field.name}>
            <Table.Td>{field.name}</Table.Td>
            <Table.Td
              style={{
                fontWeight: field.relation_ship_fields.length
                  ? "700"
                  : "normal",
              }}
            >
              {field.type}
            </Table.Td>
            <Table.Td>
              {field.constraints.map((c) => (
                <div key={c}>{c}</div>
              ))}
            </Table.Td>
            <Table.Td style={{ fontWeight: 700 }}>
              {field.relation_ship_fields.map((c) => (
                <div key={c}>{c}</div>
              ))}
            </Table.Td>
            <Table.Td style={{ fontWeight: 700 }}>
              {field.relation_ship_references.map((c) => (
                <div key={c}>{c}</div>
              ))}
            </Table.Td>
            <Table.Td>{field.is_index}</Table.Td>
          </Table.Tr>
        ))}
      </Table.Tbody>
    </Table>
  );

  const DisplayAsCollapsed = () => (
    <Flex
      style={{ border: "1px solid #FFE3E3" }}
      direction={"column"}
      gap={"0"}
    >
      {model.fields.slice(0, 3).map((f) => (
        <Text
          style={{ borderBottom: "1px solid", borderColor: "#FFE3E3" }}
          key={f.name}
          p={"4px"}
        >
          {f.name}
        </Text>
      ))}
      <Text
        style={{ borderBottom: "1px solid", borderColor: "#FFE3E3" }}
        p={"4px"}
      >
        ...
      </Text>
    </Flex>
  );

  const DisplayAs = {
    table: <DisplayAsTable />,
    code: <DisplayAsCode />,
    collapsed: <DisplayAsCollapsed />,
  };
  const [displayAs, setDisplayAs] =
    useState<keyof typeof DisplayAs>("collapsed");

  useEffect(() => {
    if (displayAs !== "collapsed") {
      setActiveModel(model.id);
    }
  }, [displayAs]);

  useEffect(() => {
    if (activeModel !== model.id) {
      setDisplayAs("collapsed");
    }
  }, [activeModel]);

  return (
    <Grid.Col span={model.id === activeModel ? "auto" : 2} key={model.name}>
      <ModelCaption />
      {DisplayAs[displayAs]}
      <Flex
        p={"8px"}
        style={{
          borderBottomLeftRadius: "8px",
          borderBottomRightRadius: "8px",
        }}
        bg={"dark.6"}
        gap={"sm"}
      >
        <CollapsedIcon />
        <TableIcon />
        <CodeIcon />
        <RelationShipIcon />
      </Flex>
    </Grid.Col>
  );
};
