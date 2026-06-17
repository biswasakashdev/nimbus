"use client";
import React from "react";
import {
  Folder,
  FileText,
  FileImage,
  FileCode,
  FileSpreadsheet,
  MoreVertical,
  ChevronRight,
} from "lucide-react";
import { FileNode } from "@/types/files.types";

interface ContentGridProps {
  currentItems: FileNode[];
  onFolderClick: (id: string) => void;
  breadcrumbs: { name: string; id: string | null }[];
  onBreadcrumbClick: (id: string | null) => void;
}

export const ContentGrid: React.FC<ContentGridProps> = ({
  currentItems,
  onFolderClick,
  breadcrumbs,
  onBreadcrumbClick,
}) => {
  // Helper to match file extensions to appropriate UI icons
  const getFileIcon = (node: FileNode) => {
    if (node.type === "directory")
      return <Folder className="h-10 w-10 text-amber-400 fill-amber-400" />;

    const mime = node.mimeType || "";
    if (mime.startsWith("image/"))
      return <FileImage className="h-10 w-10 text-emerald-500" />;
    if (mime.includes("spreadsheet") || mime.includes("excel"))
      return <FileSpreadsheet className="h-10 w-10 text-green-600" />;
    if (mime.includes("script") || mime.includes("json"))
      return <FileCode className="h-10 w-10 text-purple-500" />;
    return <FileText className="h-10 w-10 text-blue-500" />;
  };

  return (
    <div className="flex-1 overflow-y-auto p-6 bg-white">
      {/* Breadcrumbs navigation header */}
      <nav className="flex items-center space-x-1 text-sm text-slate-500 mb-6">
        {breadcrumbs.map((crumb, idx) => (
          <React.Fragment key={idx}>
            {idx > 0 && <ChevronRight className="h-4 w-4 text-slate-400" />}
            <button
              onClick={() => onBreadcrumbClick(crumb.id)}
              className={`hover:text-slate-800 transition-colors ${idx === breadcrumbs.length - 1 ? "text-slate-800 font-semibold" : ""}`}
            >
              {crumb.name}
            </button>
          </React.Fragment>
        ))}
      </nav>

      {/* Grid Canvas */}
      {currentItems.length === 0 ? (
        <div className="h-64 flex flex-col items-center justify-center border-2 border-dashed border-slate-200 rounded-xl p-4">
          <p className="text-slate-400 text-sm">
            This directory is empty. Drop files or use the upload button.
          </p>
        </div>
      ) : (
        <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
          {currentItems.map((item) => (
            <div
              key={item.id}
              onDoubleClick={() =>
                item.type === "directory" && onFolderClick(item.id)
              }
              className="group border border-slate-200 hover:border-blue-400 rounded-xl p-4 transition-all hover:shadow-sm bg-slate-50/30 hover:bg-white select-none cursor-pointer flex flex-col justify-between h-36"
            >
              <div className="flex items-start justify-between">
                {getFileIcon(item)}
                <button className="opacity-0 group-hover:opacity-100 p-1 hover:bg-slate-100 rounded text-slate-500 transition-all">
                  <MoreVertical className="h-4 w-4" />
                </button>
              </div>

              <div className="mt-4">
                <h4
                  className="text-sm font-medium text-slate-800 truncate"
                  title={item.name}
                >
                  {item.name}
                </h4>
                <div className="flex items-center justify-between text-xs text-slate-400 mt-1">
                  <span>
                    {item.type === "directory" ? "Folder" : item.size}
                  </span>
                  <span>{item.updatedAt}</span>
                </div>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
};
