
interface ErrorInfo {
    'code': number,
    'message': string
}

type RemoteResult<T> = { 'Ok': T } |
    { 'Err': ErrorInfo }

export class Result<T> {

    constructor(result: Promise<RemoteResult<T>>) {
        this._result = result;
    }

    protected _result: Promise<RemoteResult<T>>;

    public async unwrap(): Promise<T> {
        const result = await this._result;
        if ('Err' in result) {
            throw new Error(result.Err.message);
        }
        return result.Ok;
    }

    public async unwrapErr(): Promise<ErrorInfo> {
        const result = await this._result;
        if ('Err' in result) {
            return result.Err;
        }
        throw new Error('Result is Ok');
    }
}


type OptionalOk<T> = [] | [T]

export class OptionData<T> {
    constructor(private _value: OptionalOk<T>) {
    }

    public unwrap(): T {
        if (this._value.length === 0) {
            throw new Error('Option is empty');
        }
        return this._value[0] as T;
    }


    public isEmpty(): boolean {
        return this._value.length === 0;
    }
}

type OptionalRemoteResult<T> =
    { 'Ok' : [] | [T] } |
    { 'Err' : ErrorInfo };


export class OptionalResult<T> {

    constructor(result: Promise<OptionalRemoteResult<T>>) {
        this._result = result;
    }

    protected _result: Promise<OptionalRemoteResult<T>>;

    public async unwrap(): Promise<T> {
        const result = await this._result;
        if ('Err' in result) {
            throw new Error(result.Err.message);
        }
        if (result.Ok as [T]) {
            return result.Ok[0] as T;
        }
        throw new Error('Option is empty');
    }

}