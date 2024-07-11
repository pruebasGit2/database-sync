//import { DatabaseClient } from "./generated/DatabaseServiceClientPb";
///import { Empty } from "google-protobuf/google/protobuf/empty_pb";

//const PROTO_PATH = path.join(__dirname, '../../protos/audio.proto');

//const client = new DatabaseClient("[::1]:777");
import { GrpcWebFetchTransport } from "@protobuf-ts/grpcweb-transport";
import { DatabaseClient } from "../../gen/database/v1/database.client";
import { GetScriptsRequest } from "../../gen/database/v1/database";
import toast from "react-hot-toast";
import { RpcError } from "@protobuf-ts/runtime-rpc";

const transport = new GrpcWebFetchTransport({
    baseUrl: "http://192.168.10.12:3500",
    format: "binary"
});

const client = new DatabaseClient(transport);
export class DatabaseService {

    static getDatabases(connectionString: string): Promise<string[]> {
        return new Promise<string[]>((res) => {
            const databasesProm = client.getDatabases({connectionString});

            toast.promise(databasesProm.response.wait(1000), {
                loading: 'Fetching databases...',
                error: (_err: RpcError) => {
                    return <span className="text-xs">{_err.message.replaceAll("%20", " ")}</span>;
                },
                success: (_res) => {
                    res(_res.database);
                    return `Fetched ${_res.database.length} databases`
                }
            });
        })
    }

    static getScripts(request: GetScriptsRequest) {
        return client.getScripts(request);
    }

}


