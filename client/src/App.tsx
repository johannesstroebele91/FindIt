import './App.css'
import {Flex, Layout} from "antd";
import {Apartment} from "./types.ts";
import ApartmentList from "./components/apartment.tsx";
import React from "react";

const {Content} = Layout;

const App: React.FC = () => {

    return (
        <Layout style={{ minHeight: '100vh' }}>
            <Content style={{padding: '20px 40px'}}>
                <h1>findit</h1>
                <Flex justify={'space-between'} align={'flex-start'}>
                    {mockApartments.map((apartment) => <ApartmentList  apartment={apartment}/>)}
                </Flex>
            </Content>
        </Layout>
    );
};

export default App

const mockApartments: Apartment[] = [
    {
        id: 1,
        name: "Apartment 1",
        rooms: [
            {
                id: 1,
                name: "Living Room",
                items: [
                    {
                        id: 1,
                        name: "Sofa",
                        description: "Comfortable sofa for relaxing",
                        lastModified: new Date()
                    },
                    {
                        id: 2,
                        name: "Coffee Table",
                        description: "Stylish coffee table for your living room",
                        lastModified: new Date()
                    }
                ]
            },
            {
                id: 2,
                name: "Bedroom",
                items: [
                    {
                        id: 3,
                        name: "Bed",
                        description: "Cozy bed for a good night's sleep",
                        lastModified: new Date()
                    },
                    {
                        id: 4,
                        name: "Dresser",
                        description: "Spacious dresser for your clothes",
                        lastModified: new Date()
                    }
                ]
            }
        ]
    },
    {
        id: 2,
        name: "Apartment 2",
        rooms: [
            {
                id: 3,
                name: "Kitchen",
                items: [
                    {
                        id: 5,
                        name: "Refrigerator",
                        description: "Large refrigerator to store your groceries",
                        lastModified: new Date()
                    },
                    {
                        id: 6,
                        name: "Oven",
                        description: "Modern oven for cooking delicious meals",
                        lastModified: new Date()
                    }
                ]
            },
            {
                id: 4,
                name: "Bathroom",
                items: [
                    {
                        id: 7,
                        name: "Shower",
                        description: "Refreshing shower for a quick wash",
                        lastModified: new Date()
                    },
                    {
                        id: 8,
                        name: "Toilet",
                        description: "Comfortable toilet for your convenience",
                        lastModified: new Date()
                    }
                ]
            }
        ]
    }
];
