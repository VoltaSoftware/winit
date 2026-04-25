use objc2::{declare_class, mutability, ClassType, DeclaredClass};
use objc2_foundation::{MainThreadMarker, NSObject, NSObjectProtocol};
use objc2_ui_kit::{
    UIResponder, UIScene, UISceneConnectionOptions, UISceneDelegate, UISceneSession, UIWindowScene,
    UIWindowSceneDelegate,
};

use super::app_state;

declare_class!(
    #[derive(Debug)]
    pub(crate) struct WinitSceneDelegate;

    unsafe impl ClassType for WinitSceneDelegate {
        #[inherits(NSObject)]
        type Super = UIResponder;
        type Mutability = mutability::MainThreadOnly;
        const NAME: &'static str = "WinitSceneDelegate";
    }

    impl DeclaredClass for WinitSceneDelegate {}

    unsafe impl NSObjectProtocol for WinitSceneDelegate {}

    unsafe impl UISceneDelegate for WinitSceneDelegate {
        #[method(scene:willConnectToSession:options:)]
        fn scene_will_connect_to_session_options(
            &self,
            scene: &UIScene,
            _session: &UISceneSession,
            _connection_options: &UISceneConnectionOptions,
        ) {
            if !scene.is_kind_of::<UIWindowScene>() {
                return;
            }

            let mtm = MainThreadMarker::from(self);
            let window_scene = unsafe { &*(scene as *const UIScene as *const UIWindowScene) };
            app_state::scene_connected(mtm, window_scene);
        }

        #[method(sceneDidDisconnect:)]
        fn scene_did_disconnect(&self, scene: &UIScene) {
            if !scene.is_kind_of::<UIWindowScene>() {
                return;
            }

            let mtm = MainThreadMarker::from(self);
            let window_scene = unsafe { &*(scene as *const UIScene as *const UIWindowScene) };
            app_state::scene_disconnected(mtm, window_scene);
        }
    }

    unsafe impl UIWindowSceneDelegate for WinitSceneDelegate {}
);

pub(crate) fn register_scene_delegate_class() {
    let _ = WinitSceneDelegate::class();
}
