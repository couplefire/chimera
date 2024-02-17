interface LogoMap {
    [key: string]: string;
}
  
const logoMap: LogoMap = {
    txt: '/assets/logos/txtLogo.svg',
    pdf: '/assets/logos/pdfLogo.svg',
    // Add more mappings as needed
};

export const getLogoByExtension = (extension: string | undefined): string => {
    if (!extension) {
        return '/assets/logos/defaultLogo.svg';
    }
    return logoMap[extension.toLowerCase()] || '/assets/logos/defaultLogo.svg';
};