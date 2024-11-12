import {useProfile} from "./useProfile";

export const ProfilePage = () => {
  const {data} = useProfile({})
  console.log('data', data)
  return (
    <div>Profile</div>
  )
}