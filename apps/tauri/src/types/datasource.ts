export type DataSource = {
    name: string,
    bin: string,
    host: string,
    port: number,
    user: string,
    password: string,
    is_active: boolean,
    is_ssh: boolean,
}

export type DatasourceConfig = {
    active_ds: string,
    datasources: DataSource[]
}

export type DownloadEvent =
    | {
    event: 'started';
    data: {
        msg: string;
    };
}
    | {
    event: 'progress';
    data: {
        msg: string;
    };
}
    | {
    event: 'finished';
    data: {
        msg: string;
    };
};

export type BinaryInfo = {
    arq: string,
    version: string,
    binary: string,
}
