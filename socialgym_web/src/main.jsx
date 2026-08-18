import {StrictMode} from 'react'
import {Provider} from 'react-redux';
import {createRoot} from 'react-dom/client'
import './index.scss'
import './i18n.js';
import {store} from './redux/store/configureStore.js';
import App from "./App.jsx";
import {BrowserRouter} from "react-router";

createRoot(document.getElementById('root')).render(
    <StrictMode>
        <Provider store={store}>
            <BrowserRouter>
                <App/>
            </BrowserRouter>
        </Provider>
    </StrictMode>,
)
