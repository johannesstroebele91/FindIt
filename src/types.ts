export interface Apartment {
    id: number;
    name: string;
    rooms: Room[];
}

export interface Room {
    id: number;
    name: string;
    items: Item[];
}

export interface Item {
    id: number;
    name: string;
    description: string;
    lastModified: Date;
}
