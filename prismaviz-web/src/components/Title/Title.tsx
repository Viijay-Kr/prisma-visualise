import { Title as Heading } from "@mantine/core";
import styles from "./Text.module.css";

export const Title = () => {
  return (
    <Heading className={styles.title} order={1}>
      Prisma Viz
    </Heading>
  );
};
