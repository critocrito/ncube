import Humanize from "humanize-plus";
import React, {useEffect, useState} from "react";

import {showFileMeta} from "../lib/http";
import {FileMetadata} from "../types";

interface FileMetaProps {
  workspace: string;
  location: string;
}

const FileMeta = ({workspace, location}: FileMetaProps) => {
  const [fileMeta, setFileMeta] = useState<FileMetadata>();
  useEffect(() => {
    const f = async () => {
      const data = await showFileMeta(workspace, location);
      setFileMeta(data);
    };
    f();
  }, [workspace, location]);

  if (!fileMeta) return <div />;

  return (
    <span className="text-sm">{Humanize.fileSize(fileMeta.size_in_bytes)}</span>
  );
};

export default FileMeta;
