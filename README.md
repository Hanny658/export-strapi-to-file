# Exporting a Collection Type to File
Generally just simply exporting a title and its fields from strapi CMS backend to a file with specified format by reading its API responses


### API Doc with examples
<b>/export [POST]</b><br />
Takes 2 piece of data: 'format' specifying the format for export and 'data_name' stating the name of Collection type name in small case plural.
```bash
{
    "format": "pdf",
    "data_name": "advertisements"
}
```
Success Message: ```[format] file for [Collection Type] created successfully.```
<br/>Sample Call from React-App:
```bash
function DownloadButton({ format, dataName }) {
    const handleDownload = async () => {
        try {
            // Define the API URL
            const apiUrl = 'http://localhost:3000/export';

            // Make a POST request to initiate the file export/download
            const response = await fetch(apiUrl, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    format: format,
                    data_name: dataName,
                }),
            });

            // Check if the response is OK
            if (response.ok) {
                // Extract filename from the response header
                const disposition = response.headers.get('Content-Disposition');
                const filename = disposition
                    ? disposition.split('filename=')[1].replace(/\"/g, '')
                    : 'downloaded-file';

                // Create a Blob from the response body
                const blob = await response.blob();
                const url = window.URL.createObjectURL(blob);

                // Create a temporary anchor element and click it to download the file
                const a = document.createElement('a');
                a.href = url;
                a.download = filename;
                document.body.appendChild(a);
                a.click();

                // Cleanup
                a.remove();
                window.URL.revokeObjectURL(url);
            } else {
                console.error('File download failed:', response.statusText);
                alert('Error downloading file. Please try again.');
            }
        } catch (error) {
            console.error('Error:', error);
            alert('An error occurred while downloading the file.');
        }
    };

    return (
        <button onClick={handleDownload}>
            Download {format === 'pdf' ? 'PDF' : 'Excel'}
        </button>
    );
}

export default DownloadButton;
```

### Need to have your own .env file to record:
- Your Backend Host
- Your API Token
- The Port numbe u wish the program to listen

<b>Example .env file</b>
```bash
STRAPI_ENDPOINT=https://your-strapi-domain-name.com/api/
API_KEY=algorithms_are_fun_1233hu45o0b3f570s99
PORT=3001
```


## Little things to note:
- Character set not manually defined, better not adding non-ASCII chars (Otherwise you might see "锟斤拷烫烫烫")
- Default (or fall-back) port number is 3000
- NO useful fall-back value for the Strapi domain name and Strapi API Token
- Currently format for expoting only supports Excel ("excel") and PDF ("pdf") in the POST request
- Make sure your .env in at root directory
- == To be continued... => (BGM: roundabout)


<br />
<div align="center">
Created by <b>Hanny</b> as a Project for <b>FUN</b>, I'll be glad if this simple small thing could boost your big projects.<br />
Last Edit 17/10/2024<br />
Last but not least (as always) << <b>Algorithms are FUN!!!</b> >>
</div>
