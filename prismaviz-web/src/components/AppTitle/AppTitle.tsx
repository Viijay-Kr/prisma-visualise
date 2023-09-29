import { Title } from "../Title/Title";
import styles from "./AppTitle.module.scss";

export const AppTitle = (props: React.ComponentProps<typeof Title>) => {
  return <Title {...props} className={styles.title} />;
};
