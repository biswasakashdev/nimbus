"use client";

import React, { useState } from "react";
import {
  Folder,
  FolderOpen,
  ChevronRight,
  ChevronDown,
  HardDrive,
  Clock,
  Star,
  Trash2,
} from "lucide-react";
import { FileNode } from "@/types/files.types";

interface SidebarNavProps {
  data: FileNode[];
  onSelectFolder: (folderId: string | null) => void;
  currentFolderId: string | null;
}

export const SidebarNav: React.FC<SidebarNavProps> = ({
  data,
  onSelectFolder,
  currentFolderId,
}) => {
  return (
    <aside className="w-64 border-r bg-slate-50/50 p-4 flex flex-col h-screen justify-between">
      <div className="space-y-6">
        <div className="flex items-center space-x-2 px-2 py-1">
          <HardDrive className="h-6 w-6 text-blue-600" />
          <span className="font-semibold text-lg tracking-tight">Nimbus</span>
        </div>

        <div className="space-y-1">
          <button
            onClick={() => onSelectFolder(null)}
            className={`w-full flex items-center space-x-3 px-3 py-2 text-sm rounded-lg transition-colors ${!currentFolderId ? "bg-blue-50 text-blue-700 font-medium" : "text-slate-600 hover:bg-slate-100"}`}
          >
            <HardDrive className="h-4 w-4" />
            <span>My Drive</span>
          </button>

          {/* Render the File Tree */}
          <div className="pl-4 mt-2 space-y-1">
            {data
              .filter((node) => node.type === "directory")
              .map((folder) => (
                <TreeItem
                  key={folder.id}
                  node={folder}
                  onSelect={onSelectFolder}
                  currentFolderId={currentFolderId}
                />
              ))}
          </div>
        </div>

        <hr className="border-slate-200" />

        <div className="space-y-1">
          <button className="w-full flex items-center space-x-3 px-3 py-2 text-sm rounded-lg text-slate-600 hover:bg-slate-100">
            <Clock className="h-4 w-4" />
            <span>Recent</span>
          </button>
          <button className="w-full flex items-center space-x-3 px-3 py-2 text-sm rounded-lg text-slate-600 hover:bg-slate-100">
            <Star className="h-4 w-4" />
            <span>Starred</span>
          </button>
          <button className="w-full flex items-center space-x-3 px-3 py-2 text-sm rounded-lg text-slate-600 hover:bg-slate-100">
            <Trash2 className="h-4 w-4" />
            <span>Trash</span>
          </button>
        </div>
      </div>
    </aside>
  );
};

// Internal Recursive Tree Component
const TreeItem: React.FC<{
  node: FileNode;
  onSelect: (id: string) => void;
  currentFolderId: string | null;
}> = ({ node, onSelect, currentFolderId }) => {
  const [isOpen, setIsOpen] = useState(false);
  const hasDirectories = node.children?.some(
    (child) => child.type === "directory",
  );
  const isSelected = currentFolderId === node.id;

  return (
    <div className="space-y-1">
      <div
        className={`flex items-center justify-between w-full rounded-md px-2 py-1.5 text-sm transition-colors ${isSelected ? "bg-blue-100/70 text-blue-800 font-medium" : "text-slate-600 hover:bg-slate-100"}`}
      >
        <button
          className="flex items-center space-x-2 flex-1 text-left"
          onClick={() => onSelect(node.id)}
        >
          {isOpen ? (
            <FolderOpen className="h-4 w-4 text-blue-500" />
          ) : (
            <Folder className="h-4 w-4 text-blue-500" />
          )}
          <span className="truncate">{node.name}</span>
        </button>

        {hasDirectories && (
          <button
            onClick={() => setIsOpen(!isOpen)}
            className="p-0.5 hover:bg-slate-200 rounded"
          >
            {isOpen ? (
              <ChevronDown className="h-3 w-3" />
            ) : (
              <ChevronRight className="h-3 w-3" />
            )}
          </button>
        )}
      </div>

      {isOpen && node.children && (
        <div className="pl-4 border-l border-slate-200 ml-3 space-y-1">
          {node.children
            .filter((child) => child.type === "directory")
            .map((child) => (
              <TreeItem
                key={child.id}
                node={child}
                onSelect={onSelect}
                currentFolderId={currentFolderId}
              />
            ))}
        </div>
      )}
    </div>
  );
};
