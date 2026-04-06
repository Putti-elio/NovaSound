-- Example data for artists table - Popular Rappers
-- Run this SQL to insert sample rappers into the database

INSERT OR IGNORE INTO artists (id, name, image_path) VALUES
('550e8400-e29b-41d4-a716-446655440000', 'Kendrick Lamar', '/images/artists/kendrick_lamar.jpg'),
('550e8400-e29b-41d4-a716-446655440001', 'J. Cole', '/images/artists/j_cole.jpg'),
('550e8400-e29b-41d4-a716-446655440002', 'Drake', '/images/artists/drake.jpg'),
('550e8400-e29b-41d4-a716-446655440003', 'Travis Scott', '/images/artists/travis_scott.jpg'),
('550e8400-e29b-41d4-a716-446655440004', 'Eminem', '/images/artists/eminem.jpg'),
('550e8400-e29b-41d4-a716-446655440005', 'Kanye West', '/images/artists/kanye_west.jpg'),
('550e8400-e29b-41d4-a716-446655440006', 'Jay-Z', '/images/artists/jay_z.jpg'),
('550e8400-e29b-41d4-a716-446655440007', 'Nas', '/images/artists/nas.jpg'),
('550e8400-e29b-41d4-a716-446655440008', 'Tyler, The Creator', '/images/artists/tyler_the_creator.jpg'),
('550e8400-e29b-41d4-a716-446655440009', 'ASAP Rocky', '/images/artists/asap_rocky.jpg'),
('550e8400-e29b-41d4-a716-446655440010', 'Post Malone', '/images/artists/post_malone.jpg'),
('550e8400-e29b-41d4-a716-446655440011', 'Lil Baby', '/images/artists/lil_baby.jpg'),
('550e8400-e29b-41d4-a716-446655440012', '21 Savage', '/images/artists/21_savage.jpg'),
('550e8400-e29b-41d4-a716-446655440013', 'Metro Boomin', '/images/artists/metro_boomin.jpg'),
('550e8400-e29b-41d4-a716-446655440014', 'Future', '/images/artists/future.jpg');

-- Albums data - Linked to artists
INSERT OR IGNORE INTO albums (id, name, total_duration, release_date, artist_id, image_path) VALUES
-- Kendrick Lamar albums
('660e8400-e29b-41d4-a716-446655440000', 'good kid, m.A.A.d city', 0, 1350950400, '550e8400-e29b-41d4-a716-446655440000', '/images/albums/good_kid_maad_city.jpg'),
('660e8400-e29b-41d4-a716-446655440001', 'To Pimp a Butterfly', 0, 1426636800, '550e8400-e29b-41d4-a716-446655440000', '/images/albums/to_pimp_a_butterfly.jpg'),
('660e8400-e29b-41d4-a716-446655440002', 'DAMN.', 0, 1492560000, '550e8400-e29b-41d4-a716-446655440000', '/images/albums/damn.jpg'),
('660e8400-e29b-41d4-a716-446655440003', 'Mr. Morale & The Big Steppers', 0, 1651536000, '550e8400-e29b-41d4-a716-446655440000', '/images/albums/mr_morale.jpg'),

-- J. Cole albums
('660e8400-e29b-41d4-a716-446655440004', '2014 Forest Hills Drive', 0, 1418860800, '550e8400-e29b-41d4-a716-446655440001', '/images/albums/forest_hills_drive.jpg'),
('660e8400-e29b-41d4-a716-446655440005', '4 Your Eyez Only', 0, 1481673600, '550e8400-e29b-41d4-a716-446655440001', '/images/albums/4_your_eyez_only.jpg'),
('660e8400-e29b-41d4-a716-446655440006', 'KOD', 0, 1524096000, '550e8400-e29b-41d4-a716-446655440001', '/images/albums/kod.jpg'),

-- Drake albums
('660e8400-e29b-41d4-a716-446655440007', 'Take Care', 0, 1320710400, '550e8400-e29b-41d4-a716-446655440002', '/images/albums/take_care.jpg'),
('660e8400-e29b-41d4-a716-446655440008', 'Nothing Was The Same', 0, 1379376000, '550e8400-e29b-41d4-a716-446655440002', '/images/albums/nothing_was_the_same.jpg'),
('660e8400-e29b-41d4-a716-446655440009', 'Views', 0, 1461542400, '550e8400-e29b-41d4-a716-446655440002', '/images/albums/views.jpg'),

-- Travis Scott albums
('660e8400-e29b-41d4-a716-446655440010', 'Rodeo', 0, 1441065600, '550e8400-e29b-41d4-a716-446655440003', '/images/albums/rodeo.jpg'),
('660e8400-e29b-41d4-a716-446655440011', 'Astroworld', 0, 1534291200, '550e8400-e29b-41d4-a716-446655440003', '/images/albums/astroworld.jpg'),
('660e8400-e29b-41d4-a716-446655440012', 'Utopia', 0, 1690156800, '550e8400-e29b-41d4-a716-446655440003', '/images/albums/utopia.jpg'),

-- Eminem albums
('660e8400-e29b-41d4-a716-446655440013', 'The Marshall Mathers LP', 0, 956880000, '550e8400-e29b-41d4-a716-446655440004', '/images/albums/marshall_mathers_lp.jpg'),
('660e8400-e29b-41d4-a716-446655440014', 'The Eminem Show', 0, 1022457600, '550e8400-e29b-41d4-a716-446655440004', '/images/albums/eminem_show.jpg'),
('660e8400-e29b-41d4-a716-446655440015', 'Recovery', 0, 1276204800, '550e8400-e29b-41d4-a716-446655440004', '/images/albums/recovery.jpg'),

-- Kanye West albums
('660e8400-e29b-41d4-a716-446655440016', 'My Beautiful Dark Twisted Fantasy', 0, 1289174400, '550e8400-e29b-41d4-a716-446655440005', '/images/albums/mbdtf.jpg'),
('660e8400-e29b-41d4-a716-446655440017', 'Yeezus', 0, 1370995200, '550e8400-e29b-41d4-a716-446655440005', '/images/albums/yeezus.jpg'),
('660e8400-e29b-41d4-a716-446655440018', 'Donda', 0, 1630368000, '550e8400-e29b-41d4-a716-446655440005', '/images/albums/donda.jpg'),

-- Jay-Z albums
('660e8400-e29b-41d4-a716-446655440019', 'The Blueprint', 0, 999820800, '550e8400-e29b-41d4-a716-446655440006', '/images/albums/blueprint.jpg'),
('660e8400-e29b-41d4-a716-446655440020', 'The Black Album', 0, 1066953600, '550e8400-e29b-41d4-a716-446655440006', '/images/albums/black_album.jpg'),
('660e8400-e29b-41d4-a716-446655440021', '4:44', 0, 1498608000, '550e8400-e29b-41d4-a716-446655440006', '/images/albums/444.jpg'),

-- Nas albums
('660e8400-e29b-41d4-a716-446655440022', 'Illmatic', 0, 799286400, '550e8400-e29b-41d4-a716-446655440007', '/images/albums/illmatic.jpg'),
('660e8400-e29b-41d4-a716-446655440023', 'Stillmatic', 0, 1006473600, '550e8400-e29b-41d4-a716-446655440007', '/images/albums/stillmatic.jpg'),
('660e8400-e29b-41d4-a716-446655440024', 'King''s Disease', 0, 1597622400, '550e8400-e29b-41d4-a716-446655440007', '/images/albums/kings_disease.jpg'),

-- Tyler, The Creator albums
('660e8400-e29b-41d4-a716-446655440025', 'Flower Boy', 0, 1502496000, '550e8400-e29b-41d4-a716-446655440008', '/images/albums/flower_boy.jpg'),
('660e8400-e29b-41d4-a716-446655440026', 'IGOR', 0, 1558828800, '550e8400-e29b-41d4-a716-446655440008', '/images/albums/igor.jpg'),
('660e8400-e29b-41d4-a716-446655440027', 'Call Me If You Get Lost', 0, 1623628800, '550e8400-e29b-41d4-a716-446655440008', '/images/albums/call_me_if_you_get_lost.jpg'),

-- A$AP Rocky albums
('660e8400-e29b-41d4-a716-446655440028', 'LONG.LIVE.A$AP', 0, 1358121600, '550e8400-e29b-41d4-a716-446655440009', '/images/albums/long_live_asap.jpg'),
('660e8400-e29b-41d4-a716-446655440029', 'AT.LONG.LAST.A$AP', 0, 1431475200, '550e8400-e29b-41d4-a716-446655440009', '/images/albums/at_long_last_asap.jpg'),
('660e8400-e29b-41d4-a716-446655440030', 'Testing', 0, 1526342400, '550e8400-e29b-41d4-a716-446655440009', '/images/albums/testing.jpg'),

-- Post Malone albums
('660e8400-e29b-41d4-a716-446655440031', 'Stoney', 0, 1481846400, '550e8400-e29b-41d4-a716-446655440010', '/images/albums/stoney.jpg'),
('660e8400-e29b-41d4-a716-446655440032', 'Beerbongs & Bentleys', 0, 1525737600, '550e8400-e29b-41d4-a716-446655440010', '/images/albums/beerbongs_bentleys.jpg'),
('660e8400-e29b-41d4-a716-446655440033', 'Hollywood''s Bleeding', 0, 1568246400, '550e8400-e29b-41d4-a716-446655440010', '/images/albums/hollywoods_bleeding.jpg'),

-- Lil Baby albums
('660e8400-e29b-41d4-a716-446655440034', 'Harder Than Ever', 0, 1526342400, '550e8400-e29b-41d4-a716-446655440011', '/images/albums/harder_than_ever.jpg'),
('660e8400-e29b-41d4-a716-446655440035', 'My Turn', 0, 1581984000, '550e8400-e29b-41d4-a716-446655440011', '/images/albums/my_turn.jpg'),
('660e8400-e29b-41d4-a716-446655440036', 'It''s Only Me', 0, 1665446400, '550e8400-e29b-41d4-a716-446655440011', '/images/albums/its_only_me.jpg'),

-- 21 Savage albums
('660e8400-e29b-41d4-a716-446655440037', 'Issa Album', 0, 1499040000, '550e8400-e29b-41d4-a716-446655440012', '/images/albums/issa_album.jpg'),
('660e8400-e29b-41d4-a716-446655440038', 'I Am > I Was', 0, 1545091200, '550e8400-e29b-41d4-a716-446655440012', '/images/albums/i_am_i_was.jpg'),
('660e8400-e29b-41d4-a716-446655440039', 'Savage Mode II', 0, 1603065600, '550e8400-e29b-41d4-a716-446655440012', '/images/albums/savage_mode_ii.jpg'),

-- Metro Boomin albums
('660e8400-e29b-41d4-a716-446655440040', 'Not All Heroes Wear Capes', 0, 1541980800, '550e8400-e29b-41d4-a716-446655440013', '/images/albums/not_all_heroes_wear_capes.jpg'),
('660e8400-e29b-41d4-a716-446655440041', 'Heroes & Villains', 0, 1671062400, '550e8400-e29b-41d4-a716-446655440013', '/images/albums/heroes_villains.jpg'),

-- Future albums
('660e8400-e29b-41d4-a716-446655440042', 'DS2', 0, 1438473600, '550e8400-e29b-41d4-a716-446655440014', '/images/albums/ds2.jpg'),
('660e8400-e29b-41d4-a716-446655440043', 'Future', 0, 1486944000, '550e8400-e29b-41d4-a716-446655440014', '/images/albums/future.jpg'),
('660e8400-e29b-41d4-a716-446655440044', 'Hndrxx', 0, 1487548800, '550e8400-e29b-41d4-a716-446655440014', '/images/albums/hndrxx.jpg');
