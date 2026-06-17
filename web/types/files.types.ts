export interface FileNode {
  id: string;
  name: string;
  type: "file" | "directory";
  mimeType?: string; // e.g., 'image/png', 'application/pdf'
  size?: string;
  updatedAt: string;
  children?: FileNode[];
}

export const mockDriveData: FileNode[] = [
  {
    id: "1",
    name: "Projects",
    type: "directory",
    updatedAt: "2026-06-15",
    children: [
      {
        id: "1-1",
        name: "NexusSphere_Architecture.pdf",
        type: "file",
        mimeType: "application/pdf",
        size: "4.2 MB",
        updatedAt: "2026-06-12",
      },
      {
        id: "1-2",
        name: "Cloudstorage_Specs.docx",
        type: "file",
        mimeType:
          "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        size: "1.8 MB",
        updatedAt: "2026-05-20",
      },
    ],
  },
  {
    id: "2",
    name: "Documents",
    type: "directory",
    updatedAt: "2026-06-10",
    children: [
      {
        id: "2-1",
        name: "Q3_Planning.xlsx",
        type: "file",
        mimeType:
          "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        size: "850 KB",
        updatedAt: "2026-06-01",
      },
    ],
  },
  {
    id: "3",
    name: "Profile_Picture.png",
    type: "file",
    mimeType: "image/png",
    size: "2.4 MB",
    updatedAt: "2026-06-16",
  },
  {
    id: "4",
    name: "Deployment_Script.sh",
    type: "file",
    mimeType: "text/x-shellscript",
    size: "12 KB",
    updatedAt: "2026-06-17",
  },
];
