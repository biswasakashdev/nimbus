"use client";

import React, { useState } from "react";
import { SidebarNav } from "@/components/home/sidebar-nav.component";
import { Header } from "@/components/home/header.component";
import { ContentGrid } from "@/components/home/content-grid";
import { mockDriveData, FileNode } from "@/types/files.types";
import { uploadFiles } from "./action";

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
  const handleFileUpload = async (files: FileList) => {
    // 1. Package files and target directory location into a FormData container
    const formData = new FormData();
    Array.from(files).forEach((file) => {
      formData.append("files", file); // Your backend will look for the 'files' key
    });

    if (currentFolderId) {
      formData.append("folderId", currentFolderId);
    }

    try {
      await uploadFiles(formData);
      // Update the current list after uploading the files.
    } catch (err) {
      console.error(err);
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
