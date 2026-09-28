export type Release = {
    tag_name: string
    name: string
    assets: Asset[]
}


export type Asset = {
    name: string
    browser_download_url: string
    size: string
    content_type: string
    installed: boolean
}