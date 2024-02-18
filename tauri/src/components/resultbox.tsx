import { SearchResult } from "../types/types";
import { getLogoByExtension } from "../utils/logoutils";
import styles from "../styles/resultbox.module.css";

interface ResultBoxProps {
  result: SearchResult;
}

function SearchBox({ result }: ResultBoxProps) {
  const file_extension = result.directory.split(".").pop();
  const logo_path = getLogoByExtension(file_extension);
  return (
    <div className={styles.container}>
      <img src={logo_path} className={styles.logo} />
      <div className={styles.textContainer}>
        <div className={styles.filename}>{result.filename}</div>
        <div className={styles.directory}>{result.directory}</div>
      </div>
    </div>
  );
}

export default SearchBox;
