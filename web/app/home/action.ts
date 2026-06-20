"use server";

import { createClient } from "@connectrpc/connect";
import { NimbusPublicService } from "@/proto-gen/nimbus_public/v1/main_pb";
import { createGrpcTransport } from "@connectrpc/connect-node";

const lb_transport = createGrpcTransport({
  baseUrl: "http://localhost:50051",
});
export async function uploadFiles(formData: FormData) {
  // TODO: Implement the logic to send bytes to the server.

  async function getNewClient() {
    const addr = "http://localhost:50052";

    const newTransport = createGrpcTransport({
      baseUrl: addr,
    });

    return createClient(NimbusPublicService, newTransport);
  }

  async function* createFileStream(
    file: File,
  ): AsyncGenerator<{ data: Uint8Array }> {
    const reader = file.stream().getReader();

    try {
      while (true) {
        const { done, value } = await reader.read();

        if (done) {
          break;
        }

        yield {
          data: value,
        };
      }
    } finally {
      reader.releaseLock();
    }
  }

  const files = formData.getAll("files") as File[];

  const res = await Promise.all(
    files.map(async (file) => {
      const client = await getNewClient();
      return client.putObject(createFileStream(file), {
        headers: {
          object_id: file.name,
          access_type: "public",
          object_type: file.size.toString(),
          meta_data: JSON.stringify({}),
        },
      });
    }),
  );

  console.log(res.length);
}
