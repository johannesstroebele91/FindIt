import React, { useState } from 'react';
import { Input, Button, List } from 'antd';

interface Item {
    id: number;
    name: string;
    description: string;
    lastModified: Date;
}

interface ItemListProps {
    item: Item;
}

const ItemList: React.FC<ItemListProps> = ({ item }) => {
    const [name, setName] = useState(item.name);
    const [description, setDescription] = useState(item.description);

    const handleNameChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        setName(e.target.value);
    };

    const handleDescriptionChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        setDescription(e.target.value);
    };

    return (
        <List.Item style={{ margin: '15px' }}>
            <Input value={name} onChange={handleNameChange} style={{ margin: '5px' }} />
            <Input value={description} onChange={handleDescriptionChange} style={{ margin: '5px' }} />
            <Button style={{ margin: '5px' }}>Edit</Button>
        </List.Item>
    );
};

export default ItemList;
