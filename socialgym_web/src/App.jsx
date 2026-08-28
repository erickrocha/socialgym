import './App.scss'
import {Spinner} from "./commons/gui/index.js";
import {lazy, Suspense} from "react";
import {Navigate, Route, Routes} from "react-router";
import ProtectedRoute from "./guard/ProtectedRoute.jsx";
import CookieBanner from './commons/components/CookieBanner/CookieBanner.jsx';

const SignIn = lazy(() => import("./pages/security/SignIn/SignIn.jsx"));
const SignUp = lazy(() => import("./pages/security/SignUp/SignUp.jsx"));

const Home = lazy(() => import("./pages/home/Home.jsx"));
const Profile = lazy(() => import("./pages/Profile/Profile.jsx"));
const PersonProfile = lazy(() => import("./pages/Profile/PersonProfile.jsx"));
const Workout = lazy(() => import("./pages/Workout/Workout.jsx"));
const Exercises = lazy(() => import("./pages/Exercises/Exercises.jsx"));
const WorkoutExecution = lazy(() => import("./pages/WorkoutExecution/WorkoutExecution.jsx"));
const Friends = lazy(() => import("./pages/Friends/Friends.jsx"));
const Business = lazy(() => import("./pages/Business/Business.jsx"));
const BusinessProfile = lazy(() => import("./pages/Business/BusinessProfile/BusinessProfile.jsx"));
const BusinessTeam = lazy(() => import("./pages/Business/BusinessTeam/BusinessTeam.jsx"));
const Gallery = lazy(() => import("./pages/Gallery/Gallery.jsx"));
const WorkoutSessions = lazy(() => import("./pages/WorkoutSessions/WorkoutSessions.jsx"));
const Evolution = lazy(() => import("./pages/Evolution/Evolution.jsx"));
const Notifications = lazy(() => import("./pages/Notifications/Notifications.jsx"));
const Settings = lazy(() => import("./pages/Settings/Settings.jsx"));
const LegalDocument = lazy(() => import("./pages/Legal/LegalDocument.jsx"));
const Moderation = lazy(() => import("./pages/Moderation/Moderation.jsx"));

function App() {

    return (
        <Suspense fallback={<Spinner/>}>
            <CookieBanner />
            <Routes>
                <Route path="/" element={<Navigate to="/home" replace/>}/>
                <Route path="/" element={<ProtectedRoute/>}>
                    <Route path="/home" element={<Home/>}/>
                    <Route path="/gallery" element={<Gallery/>}/>
                    <Route path="/profile" element={<Profile/>}/>
                    <Route path="/profile/:id" element={<PersonProfile/>}/>
                    <Route path="/workouts" element={<Workout/>}/>
                    <Route path="/workouts/execution/:id" element={<WorkoutExecution/>}/>
                    <Route path="/exercises" element={<Exercises/>}/>
                    <Route path="/workout-sessions" element={<WorkoutSessions/>}/>
                    <Route path="/evolution" element={<Evolution/>}/>
                    <Route path="/friends" element={<Friends/>}/>
                    <Route path="/notifications" element={<Notifications/>}/>
                    <Route path="/settings" element={<Settings/>}/>
                    <Route path="/business" element={<Business/>}/>
                    <Route path="/business/:id" element={<BusinessProfile/>}/>
                    <Route path="/business/:id/team" element={<BusinessTeam/>}/>
                    <Route path="/moderation" element={<Moderation/>}/>
                </Route>
                <Route path="/login" element={<SignIn/>}/>
                <Route path="/signup" element={<SignUp/>}/>
                <Route path="/terms" element={<LegalDocument document="terms"/>}/>
                <Route path="/privacy" element={<LegalDocument document="privacy"/>}/>
                <Route path="/privacy/dpo" element={<LegalDocument document="privacy"/>}/>
                <Route path="/health-consent" element={<LegalDocument document="health_data"/>}/>
                <Route path="*" element={<Navigate to="/home"/>}/>
            </Routes>
        </Suspense>
    )
}

export default App
