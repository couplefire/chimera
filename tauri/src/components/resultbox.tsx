import { SearchResult } from "../types/types";
import { getLogoByExtension } from "../utils/logoutils";
import { invoke } from "@tauri-apps/api";
import { appWindow } from "@tauri-apps/api/window";
import styles from "../styles/resultbox.module.css";

interface ResultBoxProps {
    result: SearchResult;
}


function SearchBox({ result }: ResultBoxProps ) {
    const file_extension = result.directory.split('.').pop() ?? 'aosdfadi'; 
    const logo_path = getLogoByExtension(file_extension, result.directory);

    const renderFileName = (fileName: string) => {
        if (fileName.length > 30) {
            return '...' + fileName.slice(-27);
        }
        return fileName;
    }

    const renderDirectory = (directory: string) => {
        const maxDisplayLength = 30;
        const parts = directory.split('/');
        let currentPath = parts.pop() || '';
        while (parts.length > 0 && currentPath.length + parts[parts.length-1].length + 1 <= maxDisplayLength) {
            currentPath = parts.pop() + '/' + currentPath;
        }
        if (currentPath.length > maxDisplayLength) {
            currentPath = '...' + currentPath.slice(-maxDisplayLength + 3);
        } else if (parts.length > 0) {
            currentPath = '...' + '/' + currentPath;
        }
        return currentPath;
    }

    const renderFileSize = (fileSize: number) => {
        if (fileSize < 1024 / 2) {
            return `${fileSize} bytes`;
        } else if (fileSize < 1024 * 1024 / 2) {
            return `${(fileSize / 1024).toFixed(2)} KB`;
        } else if (fileSize < 1024 * 1024 * 1024 / 2) {
            return `${(fileSize / 1024 / 1024).toFixed(2)} MB`;
        } else {
            return `${(fileSize / 1024 / 1024 / 1024).toFixed(2)} GB`;
        }
    }

    const openFile = () => {
        invoke('open', { path: result.directory });
        appWindow.hide();
    }

    return (
        <div className={`${styles.resultBox}`} onClick={openFile}>
            <div className={styles.container}>
                <img src={logo_path} className={styles.logo} />
                <div className={styles.textContainer}>
                    <div className={styles.topLevel}>
                        <div>{renderFileName(result.fileName)}</div>
                    </div>
                    <div className={styles.bottomLevel}>
                        <div>{renderDirectory(result.directory)}</div>
                        {result.numPages !== null && result.numPages > 0 &&
                            <div>{result.numPages} pages</div>
                        }
                        <div>{renderFileSize(result.fileSize)}</div>
                    </div>
                </div>
            </div>
        </div>
    );
}

export default SearchBox;
