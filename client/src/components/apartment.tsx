import React, { useState } from 'react';
import { Card, Button } from 'antd';
import {Apartment} from "../types.ts";
import RoomList from "./room.tsx";

interface ApartmentListProps {
    apartment: Apartment;
}

const ApartmentList: React.FC<ApartmentListProps> = ({ apartment }) => {
    const [showRooms, setShowRooms] = useState(false);

    return (
        <Card title={apartment.name} style={{ margin: '15px', width: '350px' }}>
            <Button onClick={() => setShowRooms(!showRooms)}>
                {showRooms ? 'Hide Rooms' : 'Show Rooms'}
            </Button>
            {showRooms &&  apartment.rooms.map((room) => <RoomList room={room} />)}
        </Card>
    );
};

export default ApartmentList;
