//import { DatabaseClient } from "./generated/DatabaseServiceClientPb";
///import { Empty } from "google-protobuf/google/protobuf/empty_pb";

//const PROTO_PATH = path.join(__dirname, '../../protos/audio.proto');

//const client = new DatabaseClient("[::1]:777");

export class DatabaseService {

    static getDatabases(): Promise<string[]>{
        return new Promise(() => {
            
        });
    } 

}


