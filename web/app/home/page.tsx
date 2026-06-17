"use client";

import React, { useState } from "react";
import { SidebarNav } from "@/components/home/sidebar-nav.component";
import { Header } from "@/components/home/header.component";
import { ContentGrid } from "@/components/home/content-grid";
import { mockDriveData, FileNode } from "@/types/files.types";

export const DrivePage: React.FC = () => {
  const [driveData, setDriveData] = useState<FileNode[]>(mockDriveData);
  const [currentFolderId, setCurrentFolderId] = useState<string | null>(null);

  // Helper utility to find target folder inside our data tree structure
  const findFolderById = (
    nodes: FileNode[],
    targetId: string | null,
  ): FileNode | null => {
    if (!targetId) return null;
    for (const node of nodes) {
      if (node.id === targetId) return node;
      if (node.children) {
        const found = findFolderById(node.children, targetId);
        if (found) return found;
      }
    }
    return null;
  };

  // Determine what list of contents should currently be visible on-screen
  const getCurrentContents = (): FileNode[] => {
    if (!currentFolderId) {
      return driveData; // Root Layer Contents
    }
    const currentFolder = findFolderById(driveData, currentFolderId);
    return currentFolder?.children || [];
  };

  // Generate dynamic path structure array for the breadcrumb navigator components
  const getBreadcrumbs = () => {
    const crumbs = [{ name: "My Drive", id: null as string | null }];
    if (!currentFolderId) return crumbs;

    // Fast static flat-lookup for basic simulation
    const currentFolder = findFolderById(driveData, currentFolderId);
    if (currentFolder) {
      crumbs.push({ name: currentFolder.name, id: currentFolder.id });
    }
    return crumbs;
  };

  // State Handler tracking local file upload actions
  const handleFileUpload = (files: FileList) => {
    const newFiles: FileNode[] = Array.from(files).map((file, idx) => ({
      id: `uploaded-${Date.now()}-${idx}`,
      name: file.name,
      type: "file",
      mimeType: file.type,
      size: `${(file.size / (1024 * 1024)).toFixed(1)} MB`,
      updatedAt: new Date().toISOString().split("T")[0],
    }));

    if (!currentFolderId) {
      setDriveData([...driveData, ...newFiles]);
    } else {
      const updateTree = (nodes: FileNode[]): FileNode[] => {
        return nodes.map((node) => {
          if (node.id === currentFolderId) {
            return {
              ...node,
              children: [...(node.children || []), ...newFiles],
            };
          } else if (node.children) {
            return { ...node, children: updateTree(node.children) };
          }
          return node;
        });
      };
      setDriveData(updateTree(driveData));
    }
  };

  return (
    <div className="flex h-screen w-screen overflow-hidden font-sans text-slate-900 bg-slate-50">
      {/* 1. Left Hand Side Structural Navigation Tree */}
      <SidebarNav
        data={driveData}
        currentFolderId={currentFolderId}
        onSelectFolder={setCurrentFolderId}
      />

      {/* 2. Right Canvas Area container */}
      <div className="flex-1 flex flex-col min-w-0">
        <Header onFileUpload={handleFileUpload} />

        <ContentGrid
          currentItems={getCurrentContents()}
          onFolderClick={setCurrentFolderId}
          breadcrumbs={getBreadcrumbs()}
          onBreadcrumbClick={setCurrentFolderId}
        />
      </div>
    </div>
  );
};

export default DrivePage;
