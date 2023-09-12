use x11_man::{
    ffi,
    x::{SelectInputConfig, SimpleWindow},
    xlib::{Event, TInput, XDisplay},
};

fn main() {
    let mut display = XDisplay::new(None);
    //let mut window = SimpleWindow::new(&display, 500, 300);

    unsafe {
        let root_window = ffi::xlib::RootWindow(display._d, 0);
        let mut window_atributes = ffi::xlib::XSetWindowAttributes::default();
        let mut window = ffi::xlib::XCreateWindow(
            display._d,
            root_window,
            0,
            0,
            500,
            300,
            1,
            0,
            0,
            ffi::xlib::XDefaultVisual(display._d, 0),
            0,
            &mut window_atributes,
        );

        let mut gc_values = ffi::xlib::XGCValues::default();
        let gc = ffi::xlib::XCreateGC(display._d, window, 0, &gc_values);
        ffi::xlib::XSetForeground(display._d, gc, 0xffffff);
        ffi::xlib::XSetBackground(display._d, gc, 0x000000);
        ffi::xlib::XSetFillStyle(display._d, gc, ffi::x::FillSolid as i32);
        ffi::xlib::XSetWindowBackground(display._d, window, 0x000000);

        ffi::xlib::XMapWindow(display._d, window);

        let mut select_input = SelectInputConfig::new();
        ffi::xlib::XSelectInput(
            display._d,
            window,
            (ffi::x::ExposureMask | ffi::x::DestroyNotify as u64 | ffi::x::CreateNotify as u64)
                as i64,
        );

        let mut root_return = 0;
        let mut parent_return = 0;
        let mut childrens = &mut 0u64 as *mut u64;
        let mut count = 0;

        ffi::xlib::XQueryTree(
            display._d,
            root_window,
            &mut root_return,
            &mut parent_return,
            &mut childrens,
            &mut count,
        );

        let mut selected_window = 0;

        let mut c = 0;

        println!("Count: {}", count);
        for i in 0..count as isize {
            let win = *childrens.offset(i);
            let mut name_return = &mut 0i8 as *mut i8;
            let has_name = ffi::xlib::XFetchName(display._d, win, &mut name_return);
            if has_name == 1 {
                let name = x11_man::xlib::c_str_to_string(name_return);
                println!("Window Name: {}", name);
                if name.trim() == "brave" {
                    c += 1;
                    if c == 1 {
                        println!("Brave finded!");
                        selected_window = root_window;
                        break;
                    }
                }
            }
        }

        if selected_window > 0 {
            let mut attributes = ffi::xlib::XWindowAttributes::new();

            let res = ffi::xlib::XGetWindowAttributes(display._d, root_window, &mut attributes);
            println!("Window atributes aquired: {}", res);
            println!("{:#?}", attributes);
            let mut image = ffi::xlib::XGetImage(
                display._d,
                selected_window,
                0,
                0,
                attributes.width as u32,
                attributes.height as u32,
                ffi::xlib::AllPlanes,
                ffi::x::ZPixmap as i32,
            );

            println!("Starting Loop");

            let mut event = ffi::xlib::XExposeEvent {
                _type: ffi::x::Expose,
                serial: 0,
                send_event: 0,
                display: display._d,
                window,
                x: 0,
                y: 0,
                width: 500,
                height: 500,
                count: 0,
            };
            loop {
                if let Some(event) = display.try_next_event() {
                    match &event {
                        Event::Expose(event) => {
                            //println!("Expose Event: {:?}", event);
                            ffi::xlib::XPutImage(
                                display._d,
                                window,
                                gc,
                                image,
                                0,
                                0,
                                0,
                                0,
                                attributes.width as u32,
                                attributes.height as u32,
                            );
                        }
                        Event::DestroyNotify(event) => {
                            //println!("Window Closed: {}", event.window);
                            break;
                        }
                        _ => {
                            //println!("Unused Event: {:?}", event);
                        }
                    }
                }
                std::thread::sleep(std::time::Duration::from_secs(1) / 144);
                ((*image).destroy_image)(image);
                image = ffi::xlib::XGetImage(
                    display._d,
                    selected_window,
                    0,
                    0,
                    attributes.width as u32,
                    attributes.height as u32,
                    ffi::xlib::AllPlanes,
                    ffi::x::ZPixmap as i32,
                );

                ffi::xlib::XPutImage(
                    display._d,
                    window,
                    gc,
                    image,
                    0,
                    0,
                    0,
                    0,
                    attributes.width as u32,
                    attributes.height as u32,
                );
            }
        }
        println!("Exit succesful!");
    }
}
/*
use ffi::xlib::*;
use ffi::x::*;
unsafe fn GetImage(register: *mut Display, d: Drawable, x: i32, y: i32, width: u32, height: u32, plane_mask: u64, format: i32) -> *mut XImage{
    ffi::xlib::
}*/

/*
XImage *XGetImage (
     register Display *dpy,
     Drawable d,
     int x,
     int y,
     unsigned int width,
     unsigned int height,
     unsigned long plane_mask,
     int format)	/* either XYPixmap or ZPixmap */
{
    xGetImageReply rep;
    register xGetImageReq *req;
    char *data;
    unsigned long nbytes;
    XImage *image;
    int planes;
    LockDisplay(dpy);
    GetReq (GetImage, req);
    /*
     * first set up the standard stuff in the request
     */
    req->drawable = d;
    req->x = x;
    req->y = y;
    req->width = width;
    req->height = height;
    req->planeMask = plane_mask;
    req->format = format;

    if (_XReply (dpy, (xReply *) &rep, 0, xFalse) == 0 ||
        rep.length == 0) {
        UnlockDisplay(dpy);
        SyncHandle();
        return (XImage *)NULL;
    }

    if (rep.length < (INT_MAX >> 2)) {
        nbytes = (unsigned long)rep.length << 2;
        data = Xmalloc(nbytes);
    } else
        data = NULL;
    if (! data) {
        _XEatDataWords(dpy, rep.length);
        UnlockDisplay(dpy);
        SyncHandle();
        return (XImage *) NULL;
    }
        _XReadPad (dpy, data, nbytes);
        if (format == XYPixmap) {
        image = XCreateImage(dpy, _XVIDtoVisual(dpy, rep.visual),
        Ones (plane_mask &
            (((unsigned long)0xFFFFFFFF) >> (32 - rep.depth))),
        format, 0, data, width, height, dpy->bitmap_pad, 0);
        planes = image->depth;
    } else { /* format == ZPixmap */
            image = XCreateImage (dpy, _XVIDtoVisual(dpy, rep.visual),
        rep.depth, ZPixmap, 0, data, width, height,
            _XGetScanlinePad(dpy, (int) rep.depth), 0);
        planes = 1;
    }

    if (!image) {
        Xfree(data);
    } else {
            if (planes < 1 || image->height < 1 || image->bytes_per_line < 1 ||
                INT_MAX / image->height <= image->bytes_per_line ||
                INT_MAX / planes <= image->height * image->bytes_per_line ||
                nbytes < planes * image->height * image->bytes_per_line) {
                XDestroyImage(image);
                image = NULL;
            }
    }
    UnlockDisplay(dpy);
    SyncHandle();
    return (image);
}
*/
