import React, { useState } from 'react';
import { Card, Button, List } from 'antd';
import {Room} from "../types.ts";
import ItemList from "./item.tsx";

interface RoomListProps {
    room: Room,
}

const RoomList: React.FC<RoomListProps> = ({ room }) => {
    const [showItems, setShowItems] = useState(false);

    return (
        <Card title={room.name} style={{ margin: '15px' }}>
            <Button onClick={() => setShowItems(!showItems)}>
                {showItems ? 'Hide Items' : 'Show Items'}
            </Button>
            {showItems && room.items.map((item) => {
                return (<List.Item>
                    <ItemList item={item}/>
                </List.Item>);
            })}
        </Card>
    );
};

export default RoomList;
