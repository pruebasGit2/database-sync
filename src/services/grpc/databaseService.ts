//import { DatabaseClient } from "./generated/DatabaseServiceClientPb";
///import { Empty } from "google-protobuf/google/protobuf/empty_pb";

//const PROTO_PATH = path.join(__dirname, '../../protos/audio.proto');

//const client = new DatabaseClient("[::1]:777");
import { GrpcWebFetchTransport } from "@protobuf-ts/grpcweb-transport";
import { DatabaseClient } from "../../gen/database/v1/database.client";

const transport = new GrpcWebFetchTransport({
    baseUrl: "http://192.168.10.12:3500",
    format: "binary"
});

const client = new DatabaseClient(transport);
export class DatabaseService {

    static getDatabases(connectionString: string): Promise<string[]> {
        return new Promise((res) => {
            client.getDatabases({connectionString})
                .then(d => res(d.response.database))
                .catch(e => {
                    console.log(e);
                    res([]);
                })
        });
    }

}


