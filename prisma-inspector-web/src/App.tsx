import "@mantine/core/styles.css";

import { Flex, MantineProvider, createTheme } from "@mantine/core";
import { SchemaUpload } from "./components/SchemaUpload/SchemaUpload";
import "./App.css";
import { AppTitle } from "./components/AppTitle/AppTitle";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ReactQueryDevtools } from "@tanstack/react-query-devtools";
import { DisplaySchema } from "./components/DisplaySchema/DisplaySchema";
const queryClient = new QueryClient();

const theme = createTheme({
  fontFamily: "Source Code Pro, monospace",
});
function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <MantineProvider theme={theme}>
        <Flex
          justify="center"
          align={"center"}
          direction={"column"}
          style={{ minWidth: "95vw" }}
        >
          <AppTitle order={1}>Prisma Inspector </AppTitle>
          <SchemaUpload />
        </Flex>
        <DisplaySchema />
      </MantineProvider>
      <ReactQueryDevtools initialIsOpen={false}></ReactQueryDevtools>
    </QueryClientProvider>
  );
}

export default App;
