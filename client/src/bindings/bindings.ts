/* eslint-disable */
// This file was generated by [rspc](https://github.com/oscartbeaumont/rspc). Do not edit this file manually.

export type Procedures = {
    queries: 
        { key: "config.getConfig", input: never, result: Config } | 
        { key: "ping", input: never, result: string } | 
        { key: "ping_auth", input: never, result: string } | 
        { key: "product.browse", input: BrowseParams, result: [ProductDetailed[], number] } | 
        { key: "product.files", input: string, result: string[][] } | 
        { key: "product.get", input: string, result: ProductDetailed },
    mutations: 
        { key: "config.setConfig", input: Config, result: string } | 
        { key: "config.setPassword", input: NewPasswordArgs, result: User } | 
        { key: "scan.start", input: boolean, result: number },
    subscriptions: never
};

export type CreatorRole = "VoiceActor" | "Creator" | "Illustrator" | "ScenarioWriter"

export type NewPasswordArgs = { password: string; new_password: string }

export type Config = { scan_dir: string[] }

export type BrowseParams = { sort_type: ProductSortType; sort_order: ProductSortOrder; page: number; limit: number; query: string }

export type Circle = { id: string; name: string }

export type ProductSortType = "Name" | "Date"

export type ProductDetailed = { id: string; title: string; circleId: string; price: number; sale_count: number; age: AgeCategory; released_at: string; rate_count: number; review_count: number; path: string; images: string[]; description: string | null; series: string | null; rating: number | null; created_at: string; updated_at: string; circle: Circle; genres: { productId: string; genreId: string; genre: Genre }[]; user_genres: { productId: string; genreId: string; count: number; genre: Genre }[]; creators: ProductCreator[] }

export type AgeCategory = "General" | "R15" | "Adult"

export type ProductCreator = { productId: string; creatorName: string; role: CreatorRole }

export type ProductSortOrder = "Asc" | "Desc"

export type User = { id: number; name: string; password: string; created_at: string }

export type Genre = { id: string; name: string }
