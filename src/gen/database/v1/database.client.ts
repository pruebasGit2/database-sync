// @generated by protobuf-ts 2.9.4
// @generated from protobuf file "database/v1/database.proto" (package "database", syntax proto3)
// tslint:disable
import type { RpcTransport } from "@protobuf-ts/runtime-rpc";
import type { ServiceInfo } from "@protobuf-ts/runtime-rpc";
import { Database } from "./database";
import type { Empty } from "../../google/protobuf/empty";
import type { DownloadScriptsRequest } from "./database";
import type { Script } from "./database";
import type { GetScriptsRequest } from "./database";
import type { ServerStreamingCall } from "@protobuf-ts/runtime-rpc";
import { stackIntercept } from "@protobuf-ts/runtime-rpc";
import type { Databases } from "./database";
import type { Connection } from "./database";
import type { UnaryCall } from "@protobuf-ts/runtime-rpc";
import type { RpcOptions } from "@protobuf-ts/runtime-rpc";
/**
 * @generated from protobuf service database.Database
 */
export interface IDatabaseClient {
    /**
     * @generated from protobuf rpc: GetDatabases(database.Connection) returns (database.Databases);
     */
    getDatabases(input: Connection, options?: RpcOptions): UnaryCall<Connection, Databases>;
    /**
     * @generated from protobuf rpc: GetScripts(database.GetScriptsRequest) returns (stream database.Script);
     */
    getScripts(input: GetScriptsRequest, options?: RpcOptions): ServerStreamingCall<GetScriptsRequest, Script>;
    /**
     * @generated from protobuf rpc: DownloadScripts(database.DownloadScriptsRequest) returns (google.protobuf.Empty);
     */
    downloadScripts(input: DownloadScriptsRequest, options?: RpcOptions): UnaryCall<DownloadScriptsRequest, Empty>;
}
/**
 * @generated from protobuf service database.Database
 */
export class DatabaseClient implements IDatabaseClient, ServiceInfo {
    typeName = Database.typeName;
    methods = Database.methods;
    options = Database.options;
    constructor(private readonly _transport: RpcTransport) {
    }
    /**
     * @generated from protobuf rpc: GetDatabases(database.Connection) returns (database.Databases);
     */
    getDatabases(input: Connection, options?: RpcOptions): UnaryCall<Connection, Databases> {
        const method = this.methods[0], opt = this._transport.mergeOptions(options);
        return stackIntercept<Connection, Databases>("unary", this._transport, method, opt, input);
    }
    /**
     * @generated from protobuf rpc: GetScripts(database.GetScriptsRequest) returns (stream database.Script);
     */
    getScripts(input: GetScriptsRequest, options?: RpcOptions): ServerStreamingCall<GetScriptsRequest, Script> {
        const method = this.methods[1], opt = this._transport.mergeOptions(options);
        return stackIntercept<GetScriptsRequest, Script>("serverStreaming", this._transport, method, opt, input);
    }
    /**
     * @generated from protobuf rpc: DownloadScripts(database.DownloadScriptsRequest) returns (google.protobuf.Empty);
     */
    downloadScripts(input: DownloadScriptsRequest, options?: RpcOptions): UnaryCall<DownloadScriptsRequest, Empty> {
        const method = this.methods[2], opt = this._transport.mergeOptions(options);
        return stackIntercept<DownloadScriptsRequest, Empty>("unary", this._transport, method, opt, input);
    }
}
