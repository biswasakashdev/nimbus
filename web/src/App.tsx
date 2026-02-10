import { Button } from "@/components/ui/button";
import FileInput from "@/components/FileInput";
import { CloudStorage } from "@cloudstorage/es-web";
import { useState, type ChangeEvent } from "react";

function App() {
  const cloudStorage = new CloudStorage({
    baseURL: "http://localhost:8000",
  });

  const [file, setFile] = useState<File | null>();

  const onUpload = async () => {
    if (file) {
      const fileStream = file.stream();
      await cloudStorage.saveFile(fileStream);
    }
  };

  const onFileChnageHandler = (eve: ChangeEvent<HTMLInputElement>) => {
    const files = eve.currentTarget.files;
    if (files) {
      setFile(files[0]);
    }
  };

  return (
    <div className="flex min-h-svh flex-col items-center justify-center">
      <FileInput onChangeInput={onFileChnageHandler} />
      <Button onClick={onUpload}>Click me</Button>
    </div>
  );
}

export default App;
