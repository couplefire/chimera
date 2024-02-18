import { fileIcons } from "./ext_to_icon";

export const getLogoByExtension = (extension:string, path: string | undefined): string => {
    let fileIcon = fileIcons.find((icon) => {
        return icon.fileExtensions?.includes(extension) || icon.fileNames?.some(name => {
            return path?.endsWith(name);
        })
    });
    if (fileIcon) {
        return '../public/' + fileIcon.name + '.svg';
    } else {
        return '../public/yaml.svg';
    }
};