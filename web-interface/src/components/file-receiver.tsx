import { useState } from "react"

interface Props {
    fileReceived: File[]
}

const FileReceiver = (props: Props) => {
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    const [files/*, setFiles*/] = useState(props.fileReceived)

    const filesReceived = files.map((file, index) => (
        <div key={ index }>
            <span>{ file.name }</span>
            <button>download</button>
            <button>remove</button>
        </div>
    ))

    return (
        <div className="section-container">
            { (files.length > 0) && (
                <div className="list-container">
                    { filesReceived }
                </div>
            ) }
        </div>
    )
}

export default FileReceiver
