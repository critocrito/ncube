/* eslint react/jsx-props-no-spreading: off */
import React, {useCallback} from "react";
import {useDropzone} from "react-dropzone";

import {isString} from "../../utils";

interface FileUploadProps {
  handleUpload: (data: Record<string, unknown>) => void;
  handleError: (msg: string) => void;
}

const FileUpload = ({handleUpload, handleError}: FileUploadProps) => {
  const onDrop = useCallback(
    (files: File[]) => {
      const reader = new FileReader();

      reader.addEventListener("abort", () =>
        handleError("file reading was aborted"),
      );

      reader.addEventListener("error", () =>
        handleError("file reading has failed"),
      );

      reader.addEventListener("load", () => {
        if (isString(reader.result)) {
          const content = JSON.parse(reader.result);
          handleUpload(content);
        }
      });

      files.forEach((file) => reader.readAsText(file));
    },
    [handleUpload, handleError],
  );

  const {getRootProps, getInputProps, isDragActive} = useDropzone({
    onDrop,
    multiple: false,
    accept: "application/json",
  });

  return (
    <div {...getRootProps()}>
      <input {...getInputProps()} />

      <div className="h4 ba br2 shadow-1 b--solitude b--dashed flex justify-around items-center o-50">
        <div className="w-50 tc">
          {isDragActive ? (
            <p>Drop your file here</p>
          ) : (
            <p>
              Attach a file by dragging &amp; dropping it here or click to
              select.
            </p>
          )}
        </div>
      </div>
    </div>
  );
};

export default FileUpload;
