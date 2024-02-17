interface LogoMap {
    [key: string]: string;
}
  
const logoMap: LogoMap = {
    txt: '../public/txt.svg',
    pdf: '../public/pdf.svg',
    cpp: '../public/cpp.svg',
    java: '../public/java.svg',
    javascript: '../public/javascript.svg',
    typescript: '../public/typescript.svg',
    python: '../public/python.svg',
    rust: '../public/rust.svg',
    // Add more mappings as needed
};

export const getLogoByExtension = (extension: string | undefined): string => {
    if (!extension) {
        return '../public/default.svg';
    }
    return logoMap[extension.toLowerCase()] || '../public/default.svg';
};