import "@mantine/core/styles.css";

import { MantineProvider, createTheme } from "@mantine/core";
import { SchemaUpload } from "./components/SchemaUpload/SchemaUpload";
import "./App.css";
import { Title } from "./components/Title/Title";

const theme = createTheme({
  fontFamily: "Space Mono, monospace",
});
function App() {
  return (
    <MantineProvider theme={theme}>
      <Title></Title>
      <SchemaUpload></SchemaUpload>
    </MantineProvider>
  );
}

export default App;
