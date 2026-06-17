"use client";

import React, { useRef } from "react";
import { Upload, Search, Bell, User } from "lucide-react";

interface HeaderProps {
  onFileUpload: (files: FileList) => void;
}

export const Header: React.FC<HeaderProps> = ({ onFileUpload }) => {
  const fileInputRef = useRef<HTMLInputElement>(null);

  const handleButtonClick = () => {
    fileInputRef.current?.click();
  };

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files && e.target.files.length > 0) {
      onFileUpload(e.target.files);
    }
  };

  return (
    <header className="h-16 border-b flex items-center justify-between px-6 bg-white shrink-0">
      {/* Search Input */}
      <div className="relative w-96">
        <Search className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-slate-400" />
        <input
          type="text"
          placeholder="Search in Drive..."
          className="w-full pl-10 pr-4 py-2 text-sm bg-slate-100 rounded-full focus:outline-none focus:ring-2 focus:ring-blue-500/20 focus:bg-white transition-all border border-transparent focus:border-blue-500"
        />
      </div>

      {/* Action / Upload Buttons Container */}
      <div className="flex items-center space-x-4">
        <input
          type="file"
          ref={fileInputRef}
          onChange={handleFileChange}
          multiple
          className="hidden"
        />

        {/* Shadcn UI styled primary button layout */}
        <button
          onClick={handleButtonClick}
          className="inline-flex items-center justify-center space-x-2 text-sm font-medium bg-blue-600 hover:bg-blue-700 text-white shadow px-4 py-2 rounded-full transition-all tracking-wide"
        >
          <Upload className="h-4 w-4" />
          <span>Upload File</span>
        </button>

        <button className="p-2 text-slate-500 hover:bg-slate-100 rounded-full transition-colors">
          <Bell className="h-5 w-5" />
        </button>
        <button className="p-2 text-slate-500 hover:bg-slate-100 rounded-full transition-colors border border-slate-200">
          <User className="h-5 w-5" />
        </button>
      </div>
    </header>
  );
};
